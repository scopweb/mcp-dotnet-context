use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

use crate::analyzer::ProjectAnalyzer;
use crate::config::Config;
use crate::context::ContextBuilder;
use crate::training::{SearchCriteria, TrainingManager};
use crate::types::CodePattern;

/// MCP Server implementation
pub struct Server {
    config: Config,
    training_manager: TrainingManager,
}

/// JSON-RPC Request structure
#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    #[allow(dead_code)]
    jsonrpc: String,
    id: Option<serde_json::Value>,
    method: String,
    params: Option<serde_json::Value>,
}

/// JSON-RPC Response structure
#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

/// JSON-RPC Error structure
#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

impl Server {
    pub async fn new(config: Config) -> Result<Self> {
        // Initialize training manager
        let patterns_path = config.storage.base_path.join(&config.storage.patterns_file);
        eprintln!("Looking for patterns in: {}", patterns_path.display());
        let mut training_manager = TrainingManager::new(patterns_path.clone());

        // Load existing patterns
        match training_manager.load_patterns().await {
            Ok(_) => {
                eprintln!(
                    "Successfully loaded {} patterns",
                    training_manager.get_all_patterns().len()
                );
            }
            Err(e) => {
                eprintln!(
                    "Error loading patterns from {}: {}",
                    patterns_path.display(),
                    e
                );
                eprintln!("Continuing with empty pattern database...");
            }
        }

        Ok(Self {
            config,
            training_manager,
        })
    }

    pub async fn run(mut self) -> Result<()> {
        eprintln!("MCP server starting on stdio transport");

        let stdin = tokio::io::stdin();
        let mut reader = tokio::io::BufReader::new(stdin).lines();
        let mut stdout = tokio::io::stdout();

        eprintln!("Waiting for requests...");

        // Process requests (wait for initialize from client)
        loop {
            match reader.next_line().await {
                Ok(Some(line)) => {
                    if line.trim().is_empty() {
                        continue;
                    }

                    eprintln!("Received request: {}", &line[..line.len().min(100)]);

                    match serde_json::from_str::<JsonRpcRequest>(&line) {
                        Ok(request) => {
                            // Check if this is a notification (no id field)
                            if request.id.is_none() && request.method.starts_with("notifications/")
                            {
                                eprintln!("Received notification: {}, ignoring", request.method);
                                continue;
                            }

                            let response = self.handle_request(request).await;
                            match serde_json::to_string(&response) {
                                Ok(response_str) => {
                                    eprintln!("Sending response");
                                    if let Err(e) = stdout.write_all(response_str.as_bytes()).await
                                    {
                                        eprintln!("Error writing to stdout: {}", e);
                                        break;
                                    }
                                    if let Err(e) = stdout.write_all(b"\n").await {
                                        eprintln!("Error writing newline: {}", e);
                                        break;
                                    }
                                    if let Err(e) = stdout.flush().await {
                                        eprintln!("Error flushing stdout: {}", e);
                                        break;
                                    }
                                    eprintln!(
                                        "Response sent successfully, waiting for next request..."
                                    );
                                }
                                Err(e) => {
                                    eprintln!("Error serializing response: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to parse request: {}", e);
                            let error_response = JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                id: None,
                                result: None,
                                error: Some(JsonRpcError {
                                    code: -32700,
                                    message: "Parse error".to_string(),
                                    data: Some(serde_json::json!({ "error": e.to_string() })),
                                }),
                            };

                            if let Ok(error_str) = serde_json::to_string(&error_response) {
                                let _ = stdout.write_all(error_str.as_bytes()).await;
                                let _ = stdout.write_all(b"\n").await;
                                let _ = stdout.flush().await;
                            }
                        }
                    }
                }
                Ok(None) => {
                    eprintln!("stdin closed (EOF)");
                    break;
                }
                Err(e) => {
                    eprintln!("Error reading from stdin: {}", e);
                    break;
                }
            }
        }

        eprintln!("MCP server shutting down");
        Ok(())
    }

    #[allow(dead_code)]
    async fn send_server_info(&self, stdout: &mut tokio::io::Stdout) -> Result<()> {
        let info = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "server/info",
            "params": {
                "name": self.config.server.name,
                "version": self.config.server.version,
                "capabilities": {
                    "tools": [
                        "analyze-project",
                        "get-patterns",
                        "train-pattern",
                        "search-patterns"
                    ]
                }
            }
        });

        let info_str = serde_json::to_string(&info)?;
        stdout.write_all(info_str.as_bytes()).await?;
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;

        Ok(())
    }

    async fn handle_request(&mut self, request: JsonRpcRequest) -> JsonRpcResponse {
        tracing::info!("Handling method: {}", request.method);

        let result = match request.method.as_str() {
            "initialize" => self.handle_initialize().await,
            "tools/list" => self.handle_tools_list().await,
            "tools/call" => self.handle_tool_call(request.params).await,
            "prompts/list" => self.handle_prompts_list().await,
            "resources/list" => self.handle_resources_list().await,
            _ => Err(format!("Unknown method: {}", request.method)),
        };

        match result {
            Ok(value) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(value),
                error: None,
            },
            Err(error_msg) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32603,
                    message: error_msg,
                    data: None,
                }),
            },
        }
    }

    async fn handle_initialize(&self) -> Result<serde_json::Value, String> {
        Ok(serde_json::json!({
            "protocolVersion": "2024-11-05",
            "serverInfo": {
                "name": self.config.server.name,
                "version": self.config.server.version,
            },
            "capabilities": {
                "tools": {}
            }
        }))
    }

    async fn handle_tools_list(&self) -> Result<serde_json::Value, String> {
        Ok(serde_json::json!({
            "tools": [
                {
                    "name": "analyze-project",
                    "description": "Analyze a .NET project and get intelligent context about its structure, patterns, and suggestions",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "project_path": {
                                "type": "string",
                                "description": "Path to the .NET project directory containing .csproj file"
                            }
                        },
                        "required": ["project_path"]
                    }
                },
                {
                    "name": "get-patterns",
                    "description": "Get code patterns for a specific framework and category",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "framework": {
                                "type": "string",
                                "description": "Framework name (e.g., 'blazor-server', 'aspnet-core')"
                            },
                            "category": {
                                "type": "string",
                                "description": "Pattern category (e.g., 'lifecycle', 'dependency-injection')"
                            }
                        },
                        "required": ["framework"]
                    }
                },
                {
                    "name": "search-patterns",
                    "description": "Search for patterns with advanced criteria including query text, tags, and minimum score",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "query": {
                                "type": "string",
                                "description": "Search query text (searches in title, description, and code)"
                            },
                            "framework": {
                                "type": "string",
                                "description": "Filter by framework"
                            },
                            "category": {
                                "type": "string",
                                "description": "Filter by category"
                            },
                            "tags": {
                                "type": "array",
                                "items": { "type": "string" },
                                "description": "Filter by tags"
                            },
                            "min_score": {
                                "type": "number",
                                "description": "Minimum relevance score (0.0 - 1.0)"
                            }
                        }
                    }
                },
                {
                    "name": "train-pattern",
                    "description": "Add a new code pattern to the training system",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "id": {
                                "type": "string",
                                "description": "Unique identifier for the pattern"
                            },
                            "category": {
                                "type": "string",
                                "description": "Pattern category"
                            },
                            "framework": {
                                "type": "string",
                                "description": "Target framework"
                            },
                            "version": {
                                "type": "string",
                                "description": "Framework version"
                            },
                            "title": {
                                "type": "string",
                                "description": "Pattern title"
                            },
                            "description": {
                                "type": "string",
                                "description": "Pattern description"
                            },
                            "code": {
                                "type": "string",
                                "description": "Code example"
                            },
                            "tags": {
                                "type": "array",
                                "items": { "type": "string" },
                                "description": "Pattern tags"
                            }
                        },
                        "required": ["id", "category", "framework", "title", "description", "code"]
                    }
                },
                {
                    "name": "get-statistics",
                    "description": "Get statistics about the pattern database",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                }
            ]
        }))
    }

    async fn handle_tool_call(
        &mut self,
        params: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let params = params.ok_or("Missing params")?;
        let tool_name = params["name"].as_str().ok_or("Missing tool name")?;
        let arguments = &params["arguments"];

        tracing::info!("Calling tool: {}", tool_name);

        match tool_name {
            "analyze-project" => self.tool_analyze_project(arguments).await,
            "get-patterns" => self.tool_get_patterns(arguments).await,
            "search-patterns" => self.tool_search_patterns(arguments).await,
            "train-pattern" => self.tool_train_pattern(arguments).await,
            "get-statistics" => self.tool_get_statistics().await,
            _ => Err(format!("Unknown tool: {}", tool_name)),
        }
    }

    // Tool: analyze-project
    async fn tool_analyze_project(
        &self,
        args: &serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        let project_path = args["project_path"]
            .as_str()
            .ok_or("Missing project_path")?;

        tracing::info!("Analyzing project: {}", project_path);

        // Analyze project
        let project_analyzer = ProjectAnalyzer::new(vec![]);
        let project = project_analyzer
            .analyze(PathBuf::from(project_path).as_path())
            .await
            .map_err(|e| format!("Failed to analyze project: {}", e))?;

        // Build context with patterns
        let context_builder =
            ContextBuilder::new().with_training_manager(self.training_manager.clone());

        let analysis = context_builder
            .build_analysis(project)
            .await
            .map_err(|e| format!("Failed to build analysis: {}", e))?;

        // Generate formatted context
        let context_string = context_builder.build_context_string(&analysis);

        Ok(serde_json::json!({
            "content": [{
                "type": "text",
                "text": context_string
            }],
            "isError": false
        }))
    }

    // Tool: get-patterns
    async fn tool_get_patterns(
        &self,
        args: &serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        let framework = args["framework"].as_str().ok_or("Missing framework")?;
        let category = args["category"].as_str();

        let patterns = if let Some(cat) = category {
            self.training_manager
                .search_by_framework_and_category(framework, cat)
        } else {
            let criteria = SearchCriteria {
                query: None,
                category: None,
                framework: Some(framework.to_string()),
                tags: vec![],
                min_score: 0.0,
            };
            self.training_manager
                .search_patterns(&criteria)
                .into_iter()
                .map(|(p, _)| p)
                .collect()
        };

        let mut output = String::new();
        output.push_str(&format!("# Patterns for {}\n\n", framework));

        if patterns.is_empty() {
            output.push_str("No patterns found.\n");
        } else {
            for pattern in patterns {
                output.push_str(&format!("## {}\n\n", pattern.title));
                output.push_str(&format!("**Category:** {}\n", pattern.category));
                output.push_str(&format!("**ID:** {}\n", pattern.id));
                output.push_str(&format!("{}\n\n", pattern.description));
                output.push_str("```csharp\n");
                output.push_str(&pattern.code);
                output.push_str("\n```\n\n");
                output.push_str(&format!("**Tags:** {}\n", pattern.tags.join(", ")));
                output.push_str(&format!("**Usage Count:** {}\n", pattern.usage_count));
                output.push_str(&format!(
                    "**Relevance:** {:.2}\n\n",
                    pattern.relevance_score
                ));
                output.push_str("---\n\n");
            }
        }

        Ok(serde_json::json!({
            "content": [{
                "type": "text",
                "text": output
            }],
            "isError": false
        }))
    }

    // Tool: search-patterns
    async fn tool_search_patterns(
        &self,
        args: &serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        let criteria = SearchCriteria {
            query: args["query"].as_str().map(|s| s.to_string()),
            category: args["category"].as_str().map(|s| s.to_string()),
            framework: args["framework"].as_str().map(|s| s.to_string()),
            tags: args["tags"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            min_score: args["min_score"].as_f64().unwrap_or(0.0) as f32,
        };

        let results = self.training_manager.search_patterns(&criteria);

        let mut output = String::new();
        output.push_str("# Pattern Search Results\n\n");
        output.push_str(&format!("Found {} patterns\n\n", results.len()));

        for (pattern, score) in results {
            output.push_str(&format!("## {} (Score: {:.2})\n\n", pattern.title, score));
            output.push_str(&format!(
                "**Framework:** {} | **Category:** {}\n",
                pattern.framework, pattern.category
            ));
            output.push_str(&format!("{}\n\n", pattern.description));
            output.push_str("```csharp\n");
            output.push_str(&pattern.code);
            output.push_str("\n```\n\n");
            output.push_str("---\n\n");
        }

        Ok(serde_json::json!({
            "content": [{
                "type": "text",
                "text": output
            }],
            "isError": false
        }))
    }

    // Tool: train-pattern
    async fn tool_train_pattern(
        &mut self,
        args: &serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        let pattern = CodePattern {
            id: args["id"].as_str().ok_or("Missing id")?.to_string(),
            category: args["category"]
                .as_str()
                .ok_or("Missing category")?
                .to_string(),
            framework: args["framework"]
                .as_str()
                .ok_or("Missing framework")?
                .to_string(),
            version: args["version"].as_str().unwrap_or("10.0").to_string(),
            title: args["title"].as_str().ok_or("Missing title")?.to_string(),
            description: args["description"]
                .as_str()
                .ok_or("Missing description")?
                .to_string(),
            code: args["code"].as_str().ok_or("Missing code")?.to_string(),
            tags: args["tags"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            usage_count: 0,
            relevance_score: 0.8, // Default relevance
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.training_manager.add_pattern(pattern.clone());

        // Save to disk
        self.training_manager
            .save_patterns()
            .await
            .map_err(|e| format!("Failed to save patterns: {}", e))?;

        let output = format!(
            "✅ Pattern '{}' added successfully!\n\n**ID:** {}\n**Category:** {}\n**Framework:** {}",
            pattern.title, pattern.id, pattern.category, pattern.framework
        );

        Ok(serde_json::json!({
            "content": [{
                "type": "text",
                "text": output
            }],
            "isError": false
        }))
    }

    // Tool: get-statistics
    async fn tool_get_statistics(&self) -> Result<serde_json::Value, String> {
        let stats = self.training_manager.get_statistics();

        let output = format!(
            "# Pattern Database Statistics\n\n\
            **Total Patterns:** {}\n\
            **Total Usage:** {}\n\
            **Average Relevance:** {:.2}\n\n\
            ## Categories\n{}\n\n\
            ## Frameworks\n{}",
            stats["total_patterns"],
            stats["total_usage"],
            stats["avg_relevance"],
            stats["categories"]
                .as_array()
                .map(|arr| arr
                    .iter()
                    .map(|v| format!("- {}", v.as_str().unwrap_or("")))
                    .collect::<Vec<_>>()
                    .join("\n"))
                .unwrap_or_default(),
            stats["frameworks"]
                .as_array()
                .map(|arr| arr
                    .iter()
                    .map(|v| format!("- {}", v.as_str().unwrap_or("")))
                    .collect::<Vec<_>>()
                    .join("\n"))
                .unwrap_or_default()
        );

        Ok(serde_json::json!({
            "content": [{
                "type": "text",
                "text": output
            }],
            "isError": false
        }))
    }

    async fn handle_prompts_list(&self) -> Result<serde_json::Value, String> {
        // Return empty prompts list (not implemented yet)
        Ok(serde_json::json!({
            "prompts": []
        }))
    }

    async fn handle_resources_list(&self) -> Result<serde_json::Value, String> {
        // Return empty resources list (not implemented yet)
        Ok(serde_json::json!({
            "resources": []
        }))
    }
}
