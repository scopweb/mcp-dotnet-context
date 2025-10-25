use anyhow::Result;
use crate::training::{TrainingManager, SearchCriteria};
use crate::types::{AnalysisResult, CodePattern, DotNetProject, Suggestion, SeverityLevel};

/// Builds intelligent context for AI assistants based on project analysis
pub struct ContextBuilder {
    training_manager: Option<TrainingManager>,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self {
            training_manager: None,
        }
    }

    /// Set the training manager for pattern retrieval
    pub fn with_training_manager(mut self, manager: TrainingManager) -> Self {
        self.training_manager = Some(manager);
        self
    }

    /// Build complete analysis with patterns and suggestions
    pub async fn build_analysis(&self, project: DotNetProject) -> Result<AnalysisResult> {
        // Detect framework type from packages
        let framework_type = self.detect_framework(&project);

        // Get relevant patterns
        let patterns = if let Some(ref manager) = self.training_manager {
            self.get_relevant_patterns(manager, &framework_type, &project)?
        } else {
            vec![]
        };

        // Generate suggestions
        let suggestions = self.generate_suggestions(&project, &framework_type);

        // Collect statistics
        let statistics = crate::types::Statistics {
            total_files: project.files.len(),
            total_classes: project.files.iter().map(|f| f.classes.len()).sum(),
            total_methods: project.files.iter()
                .flat_map(|f| &f.classes)
                .map(|c| c.methods.len())
                .sum(),
            total_lines: 0, // TODO: count actual lines
            framework_version: project.target_framework.clone(),
            package_count: project.packages.len(),
        };

        Ok(AnalysisResult {
            project,
            patterns,
            suggestions,
            statistics,
        })
    }

    /// Detect framework type from project packages
    fn detect_framework(&self, project: &DotNetProject) -> String {
        // Check for Blazor Server
        if project.packages.iter().any(|p| p.name.contains("AspNetCore.Components")) {
            return "blazor-server".to_string();
        }

        // Check for ASP.NET Core
        if project.packages.iter().any(|p| p.name.contains("AspNetCore")) {
            return "aspnet-core".to_string();
        }

        // Check for Entity Framework
        if project.packages.iter().any(|p| p.name.contains("EntityFrameworkCore")) {
            return "entity-framework".to_string();
        }

        // Default
        "dotnet".to_string()
    }

    /// Get relevant patterns based on project analysis
    fn get_relevant_patterns(
        &self,
        manager: &TrainingManager,
        framework: &str,
        project: &DotNetProject,
    ) -> Result<Vec<CodePattern>> {
        let mut all_patterns = Vec::new();

        // Search for framework-specific patterns
        let criteria = SearchCriteria {
            query: None,
            category: None,
            framework: Some(framework.to_string()),
            tags: vec![],
            min_score: 0.7, // Only high-quality patterns
        };

        let scored_patterns = manager.search_patterns(&criteria);

        // Detect specific needs based on project structure
        let mut additional_categories = Vec::new();

        // Check for lifecycle methods usage
        for file in &project.files {
            for class in &file.classes {
                for method in &class.methods {
                    if method.name.contains("OnInitialized") {
                        additional_categories.push("lifecycle");
                    }
                    if method.is_async {
                        additional_categories.push("async-patterns");
                    }
                }
            }
        }

        // Get patterns for detected categories
        for category in additional_categories {
            let results = manager.search_by_framework_and_category(framework, category);
            for pattern in results {
                if !all_patterns.iter().any(|p: &CodePattern| p.id == pattern.id) {
                    all_patterns.push(pattern.clone());
                }
            }
        }

        // Add scored patterns
        for (pattern, _score) in scored_patterns.into_iter().take(5) {
            if !all_patterns.iter().any(|p| p.id == pattern.id) {
                all_patterns.push(pattern.clone());
            }
        }

        Ok(all_patterns)
    }

    /// Generate suggestions based on project analysis
    fn generate_suggestions(
        &self,
        project: &DotNetProject,
        framework: &str,
    ) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        // Check for Blazor-specific issues
        if framework == "blazor-server" {
            suggestions.extend(self.check_blazor_patterns(project));
        }

        // Check for async/await patterns
        suggestions.extend(self.check_async_patterns(project));

        // Check for dependency injection usage
        suggestions.extend(self.check_di_patterns(project));

        suggestions
    }

    fn check_blazor_patterns(&self, project: &DotNetProject) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        for file in &project.files {
            for class in &file.classes {
                // Check if inherits from ComponentBase
                let is_component = class.base_class.as_ref()
                    .map(|b| b.contains("ComponentBase"))
                    .unwrap_or(false);

                if is_component {
                    // Check for synchronous OnInitialized
                    let has_sync_init = class.methods.iter()
                        .any(|m| m.name == "OnInitialized" && !m.is_async);

                    if has_sync_init {
                        suggestions.push(Suggestion {
                            severity: SeverityLevel::Warning,
                            category: "blazor-lifecycle".to_string(),
                            message: format!(
                                "Component '{}' uses synchronous OnInitialized(). Consider using OnInitializedAsync() for better performance.",
                                class.name
                            ),
                            file: Some(file.path.clone()),
                            line: None,
                        });
                    }
                }
            }
        }

        suggestions
    }

    fn check_async_patterns(&self, project: &DotNetProject) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        for file in &project.files {
            for class in &file.classes {
                for method in &class.methods {
                    // Check for async void (should be async Task)
                    if method.is_async && method.return_type == "void" {
                        suggestions.push(Suggestion {
                            severity: SeverityLevel::Warning,
                            category: "async-patterns".to_string(),
                            message: format!(
                                "Method '{}' in class '{}' is async void. Use async Task instead for proper exception handling.",
                                method.name, class.name
                            ),
                            file: Some(file.path.clone()),
                            line: None,
                        });
                    }
                }
            }
        }

        suggestions
    }

    fn check_di_patterns(&self, _project: &DotNetProject) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        // Could check for:
        // - Services that should be injected
        // - Missing DI registration
        // - Incorrect service lifetime

        // For now, just a placeholder
        suggestions.push(Suggestion {
            severity: SeverityLevel::Info,
            category: "dependency-injection".to_string(),
            message: "Consider using dependency injection for data access and external services.".to_string(),
            file: None,
            line: None,
        });

        suggestions
    }

    /// Build a formatted context string for AI consumption
    pub fn build_context_string(&self, analysis: &AnalysisResult) -> String {
        let mut context = String::new();

        context.push_str(&format!("# .NET Project Analysis\n\n"));
        context.push_str(&format!("**Project:** {}\n", analysis.project.name));
        context.push_str(&format!("**Framework:** {}\n", analysis.project.target_framework));
        context.push_str(&format!("**Language:** C# {}\n\n", analysis.project.language_version));

        // Dependencies
        context.push_str("## Dependencies\n\n");
        for package in &analysis.project.packages {
            context.push_str(&format!("- {} ({})\n", package.name, package.version));
        }
        context.push_str("\n");

        // Statistics
        context.push_str("## Project Statistics\n\n");
        context.push_str(&format!("- Total Files: {}\n", analysis.statistics.total_files));
        context.push_str(&format!("- Total Classes: {}\n", analysis.statistics.total_classes));
        context.push_str(&format!("- Total Methods: {}\n", analysis.statistics.total_methods));
        context.push_str("\n");

        // Relevant Patterns
        if !analysis.patterns.is_empty() {
            context.push_str("## Relevant Patterns\n\n");
            for pattern in &analysis.patterns {
                context.push_str(&format!("### {}\n", pattern.title));
                context.push_str(&format!("**Category:** {}\n", pattern.category));
                context.push_str(&format!("{}\n\n", pattern.description));
                context.push_str("```csharp\n");
                context.push_str(&pattern.code);
                context.push_str("\n```\n\n");
            }
        }

        // Suggestions
        if !analysis.suggestions.is_empty() {
            context.push_str("## Suggestions\n\n");
            for suggestion in &analysis.suggestions {
                let icon = match suggestion.severity {
                    SeverityLevel::Error => "❌",
                    SeverityLevel::Warning => "⚠️",
                    SeverityLevel::Info => "ℹ️",
                };
                context.push_str(&format!("{} **{}**: {}\n", icon, suggestion.category, suggestion.message));
            }
        }

        context
    }
}
