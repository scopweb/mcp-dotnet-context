use anyhow::Result;
use mcp_dotnet_context::analyzer::{CSharpAnalyzer, ProjectAnalyzer};
use std::fs;

#[tokio::test]
async fn test_csharp_analyzer() -> Result<()> {
    // Create a temporary C# file for testing
    let temp_dir = tempfile::tempdir()?;
    let cs_file = temp_dir.path().join("Sample.cs");

    let sample_code = r#"
using System;
using Microsoft.AspNetCore.Components;

namespace MyApp.Components
{
    public class WeatherForecast : ComponentBase
    {
        public string Summary { get; set; }
        public int Temperature { get; set; }

        protected override async Task OnInitializedAsync()
        {
            await LoadDataAsync();
            await base.OnInitializedAsync();
        }

        private async Task LoadDataAsync()
        {
            // Simulate loading data
            Summary = "Warm";
            Temperature = 25;
        }
    }
}
"#;

    fs::write(&cs_file, sample_code)?;

    // Analyze the file
    let mut analyzer = CSharpAnalyzer::new()?;
    let result = analyzer.analyze_file(&cs_file)?;

    println!("\n=== C# Analysis Result ===");
    println!("File: {:?}", result.path);
    println!("Namespace: {:?}", result.namespace);
    println!("Usings: {:?}", result.usings);
    println!("\nClasses found: {}", result.classes.len());

    for class in &result.classes {
        println!("\nClass: {}", class.name);
        println!("  Modifiers: {:?}", class.modifiers);
        println!("  Methods: {}", class.methods.len());

        for method in &class.methods {
            println!(
                "    - {} ({}): {}",
                method.name,
                if method.is_async { "async" } else { "sync" },
                method.return_type
            );
        }

        println!("  Properties: {}", class.properties.len());
        for prop in &class.properties {
            println!("    - {}: {}", prop.name, prop.prop_type);
        }
    }

    // Assertions
    assert_eq!(result.namespace, Some("MyApp.Components".to_string()));
    assert!(result.usings.contains(&"System".to_string()));
    assert!(result
        .usings
        .contains(&"Microsoft.AspNetCore.Components".to_string()));
    assert_eq!(result.classes.len(), 1);
    assert_eq!(result.classes[0].name, "WeatherForecast");
    assert_eq!(result.classes[0].properties.len(), 2);
    assert_eq!(result.classes[0].methods.len(), 2);

    Ok(())
}

#[tokio::test]
async fn test_project_analyzer() -> Result<()> {
    // Create a temporary project structure
    let temp_dir = tempfile::tempdir()?;
    let project_path = temp_dir.path();

    // Create a sample .csproj file
    let csproj_content = r#"<Project Sdk="Microsoft.NET.Sdk.Web">
  <PropertyGroup>
    <TargetFramework>net10.0</TargetFramework>
    <Nullable>enable</Nullable>
    <ImplicitUsings>enable</ImplicitUsings>
  </PropertyGroup>

  <ItemGroup>
    <PackageReference Include="Microsoft.AspNetCore.Components" Version="10.0.0" />
    <PackageReference Include="Microsoft.EntityFrameworkCore" Version="10.0.0" />
  </ItemGroup>
</Project>"#;

    fs::write(project_path.join("TestProject.csproj"), csproj_content)?;

    // Create some C# files
    fs::create_dir_all(project_path.join("Services"))?;
    fs::write(
        project_path.join("Services/DataService.cs"),
        "namespace MyApp.Services { public class DataService { } }",
    )?;

    // Analyze the project
    let analyzer = ProjectAnalyzer::new(vec![]);
    let result = analyzer.analyze(project_path).await?;

    println!("\n=== Project Analysis Result ===");
    println!("Project: {}", result.name);
    println!("Framework: {}", result.target_framework);
    println!("Packages: {}", result.packages.len());

    for package in &result.packages {
        println!("  - {} ({})", package.name, package.version);
    }

    // Assertions
    assert_eq!(result.name, "TestProject");
    assert_eq!(result.target_framework, "net10.0");
    assert_eq!(result.packages.len(), 2);
    assert!(result
        .packages
        .iter()
        .any(|p| p.name == "Microsoft.AspNetCore.Components"));

    Ok(())
}
