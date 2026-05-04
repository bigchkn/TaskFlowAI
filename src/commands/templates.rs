use crate::model::TaskTemplate;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn list() -> Result<()> {
    let template_dir = Path::new(".taskflow/templates/tasks");
    if !template_dir.exists() {
        println!("No templates found (directory .taskflow/templates/tasks does not exist).");
        return Ok(());
    }

    println!("{:<20} | {:<50}", "TEMPLATE", "DESCRIPTION");
    println!("{}", "-".repeat(73));

    for entry in fs::read_dir(template_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let content = fs::read_to_string(&path)?;
            let t: TaskTemplate = toml::from_str(&content)?;
            let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown");
            println!("{:<20} | {:<50}", name, t.description);
        }
    }
    Ok(())
}

pub fn show(name: &str) -> Result<()> {
    let path = Path::new(".taskflow/templates/tasks").join(format!("{}.toml", name));
    if !path.exists() {
        return Err(anyhow::anyhow!("Template '{}' not found", name));
    }

    let content = fs::read_to_string(&path)?;
    let t: TaskTemplate = toml::from_str(&content)?;

    println!("--------------------------------------------------");
    println!("TEMPLATE: {}", t.name);
    println!("Description: {}", t.description);
    println!("--------------------------------------------------");
    
    if !t.required_metadata.is_empty() {
        println!("REQUIRED METADATA:");
        for (k, v) in &t.required_metadata {
            println!("  - {}: ({})", k, v);
        }
    } else {
        println!("REQUIRED METADATA: None");
    }
    println!("--------------------------------------------------");

    if !t.default_subtasks.is_empty() {
        println!("DEFAULT SUBTASKS:");
        for sub in &t.default_subtasks {
            println!("  - {} [{:?}]", sub.title, sub.task_type);
        }
    } else {
        println!("DEFAULT SUBTASKS: None");
    }
    println!("--------------------------------------------------");

    if !t.required_designs.is_empty() {
        println!("REQUIRED DESIGNS:");
        for req in &t.required_designs {
            println!("  - {:?}", req.design_type);
        }
    } else {
        println!("REQUIRED DESIGNS: None");
    }
    println!("--------------------------------------------------");

    Ok(())
}
