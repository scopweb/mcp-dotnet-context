use anyhow::{Context, Result};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::types::{DotNetProject, NuGetPackage};

pub struct ProjectAnalyzer {
    ignore_patterns: Vec<String>,
}

impl ProjectAnalyzer {
    pub fn new(ignore_patterns: Vec<String>) -> Self {
        Self { ignore_patterns }
    }

    pub async fn analyze(&self, path: &Path) -> Result<DotNetProject> {
        // Find .csproj file
        let csproj_path = self.find_csproj(path)?;

        // Parse .csproj (XML)
        let (name, target_framework, packages) = self.parse_csproj(&csproj_path)?;

        // Find all .cs files
        let _files = self.find_csharp_files(path)?;

        Ok(DotNetProject {
            path: path.to_path_buf(),
            name,
            target_framework,
            language_version: "10.0".to_string(),
            packages,
            project_references: vec![],
            files: vec![],
        })
    }

    fn find_csproj(&self, path: &Path) -> Result<PathBuf> {
        // Search for .csproj file in the given directory
        for entry in WalkDir::new(path).max_depth(1) {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("csproj") {
                return Ok(path.to_path_buf());
            }
        }

        anyhow::bail!("No .csproj file found in directory: {}", path.display())
    }

    fn parse_csproj(&self, path: &Path) -> Result<(String, String, Vec<NuGetPackage>)> {
        let content = fs::read_to_string(path)
            .context("Failed to read .csproj file")?;

        let mut reader = Reader::from_str(&content);
        reader.trim_text(true);

        let project_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();
        let mut target_framework = String::new();
        let mut packages = Vec::new();

        let mut buf = Vec::new();
        let mut current_element = String::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    current_element = String::from_utf8_lossy(e.name().as_ref()).to_string();

                    // Parse PackageReference
                    if current_element == "PackageReference" {
                        let mut pkg_name = String::new();
                        let mut pkg_version = String::new();

                        for attr in e.attributes().flatten() {
                            let key = String::from_utf8_lossy(attr.key.as_ref());
                            let value = attr.unescape_value()?.to_string();

                            match key.as_ref() {
                                "Include" => pkg_name = value,
                                "Version" => pkg_version = value,
                                _ => {}
                            }
                        }

                        if !pkg_name.is_empty() {
                            packages.push(NuGetPackage {
                                name: pkg_name,
                                version: pkg_version,
                            });
                        }
                    }
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape()?.to_string();

                    // Capture TargetFramework value
                    if current_element == "TargetFramework" {
                        target_framework = text.trim().to_string();
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => anyhow::bail!("Error parsing XML at position {}: {:?}", reader.buffer_position(), e),
                _ => {}
            }
            buf.clear();
        }

        // Default to net10.0 if not found
        if target_framework.is_empty() {
            target_framework = "net10.0".to_string();
        }

        Ok((project_name, target_framework, packages))
    }

    fn find_csharp_files(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                // Skip common ignore directories
                let file_name = e.file_name().to_str().unwrap_or("");
                !file_name.starts_with('.')
                    && file_name != "bin"
                    && file_name != "obj"
                    && file_name != "node_modules"
            })
        {
            let entry = entry?;
            let path = entry.path();

            // Check for .cs files (including .razor.cs)
            if let Some(ext) = path.extension() {
                if ext == "cs" {
                    files.push(path.to_path_buf());
                }
            }
        }

        Ok(files)
    }
}
