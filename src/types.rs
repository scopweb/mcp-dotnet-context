use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a .NET project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DotNetProject {
    pub path: PathBuf,
    pub name: String,
    pub target_framework: String,
    pub language_version: String,
    pub packages: Vec<NuGetPackage>,
    pub project_references: Vec<PathBuf>,
    pub files: Vec<CSharpFile>,
}

/// NuGet package reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuGetPackage {
    pub name: String,
    pub version: String,
}

/// C# source file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CSharpFile {
    pub path: PathBuf,
    pub namespace: Option<String>,
    pub usings: Vec<String>,
    pub classes: Vec<ClassInfo>,
    pub interfaces: Vec<InterfaceInfo>,
}

/// Class information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassInfo {
    pub name: String,
    pub modifiers: Vec<String>,
    pub base_class: Option<String>,
    pub interfaces: Vec<String>,
    pub methods: Vec<MethodInfo>,
    pub properties: Vec<PropertyInfo>,
}

/// Interface information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceInfo {
    pub name: String,
    pub methods: Vec<MethodInfo>,
}

/// Method information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodInfo {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<Parameter>,
    pub modifiers: Vec<String>,
    pub is_async: bool,
}

/// Parameter information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
}

/// Property information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyInfo {
    pub name: String,
    pub prop_type: String,
    pub has_getter: bool,
    pub has_setter: bool,
}

/// Code pattern for training
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodePattern {
    pub id: String,
    pub category: String,
    pub framework: String,
    pub version: String,
    pub title: String,
    pub description: String,
    pub code: String,
    pub tags: Vec<String>,
    pub usage_count: usize,
    pub relevance_score: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub project: DotNetProject,
    pub patterns: Vec<CodePattern>,
    pub suggestions: Vec<Suggestion>,
    pub statistics: Statistics,
}

/// Code suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub severity: SeverityLevel,
    pub category: String,
    pub message: String,
    pub file: Option<PathBuf>,
    pub line: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SeverityLevel {
    Info,
    Warning,
    Error,
}

/// Project statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    pub total_files: usize,
    pub total_classes: usize,
    pub total_methods: usize,
    pub total_lines: usize,
    pub framework_version: String,
    pub package_count: usize,
}
