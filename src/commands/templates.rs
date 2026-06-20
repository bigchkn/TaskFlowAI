use crate::model::TaskTemplate;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct DefaultTemplate {
    pub name: &'static str,
    pub filename: &'static str,
    pub content: &'static str,
}

pub const DEFAULT_TEMPLATES: &[DefaultTemplate] = &[
    DefaultTemplate {
        name: "feature",
        filename: "feature.toml",
        content: r#"name = "Feature"
description = "Standard template for new features"

[required_metadata]
priority_reason = "string"
impact_analysis = "string"

[[default_subtasks]]
title = "Write LLD"
task_type = "research"

[[default_subtasks]]
title = "Implementation"
task_type = "feature"

[[required_designs]]
design_type = "lld"
"#,
    },
    DefaultTemplate {
        name: "research",
        filename: "research.toml",
        content: r#"name = "Research"
description = "Template for research, spikes, and architecture inquiries."

[required_metadata]
research_goal = "The primary question or objective of this research."
research_path = "Suggested: docs/research/"

[[required_designs]]
design_type = "rfc"

[[default_subtasks]]
title = "Initial Research & Discovery"
task_type = "research"

[[default_subtasks]]
title = "Document Findings"
task_type = "research"
"#,
    },
];

pub fn get_template(project_root: &Path, name: &str) -> Option<TaskTemplate> {
    // Strip optional .toml suffix to handle both name styles gracefully
    let clean_name = name.strip_suffix(".toml").unwrap_or(name);

    // 1. Try local first
    let path = project_root
        .join(".taskflow/templates/tasks")
        .join(format!("{}.toml", clean_name));
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(t) = toml::from_str(&content) {
                return Some(t);
            }
        }
    }
    // 2. Try default
    for dt in DEFAULT_TEMPLATES {
        if dt.name == clean_name {
            if let Ok(t) = toml::from_str(dt.content) {
                return Some(t);
            }
        }
    }
    None
}

pub fn list() -> Result<()> {
    let project_root = std::env::current_dir()?;
    let template_dir = project_root.join(".taskflow/templates/tasks");

    // Collect all template names and where they come from
    use indexmap::IndexMap;
    let mut templates: IndexMap<String, (String, String)> = IndexMap::new();

    // 1. Insert built-in templates first
    for dt in DEFAULT_TEMPLATES {
        if let Ok(t) = toml::from_str::<TaskTemplate>(dt.content) {
            templates.insert(dt.name.to_string(), ("Built-in".to_string(), t.description));
        }
    }

    // 2. Scan local directory if it exists
    if template_dir.exists() {
        for entry in fs::read_dir(&template_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(t) = toml::from_str::<TaskTemplate>(&content) {
                            let source = if templates.contains_key(name) {
                                "Local Override".to_string()
                            } else {
                                "Local".to_string()
                            };
                            templates.insert(name.to_string(), (source, t.description));
                        }
                    }
                }
            }
        }
    }

    // Sort templates by name to ensure consistent output ordering
    templates.sort_keys();

    println!("{:<20} | {:<20} | {:<50}", "TEMPLATE", "SOURCE", "DESCRIPTION");
    println!("{}", "-".repeat(96));

    for (name, (source, description)) in &templates {
        println!("{:<20} | {:<20} | {:<50}", name, source, description);
    }

    Ok(())
}

pub fn show(name: &str) -> Result<()> {
    let clean_name = name.strip_suffix(".toml").unwrap_or(name);
    let project_root = std::env::current_dir()?;
    let t = get_template(&project_root, clean_name)
        .ok_or_else(|| anyhow::anyhow!("Template '{}' not found", name))?;

    let path = project_root
        .join(".taskflow/templates/tasks")
        .join(format!("{}.toml", clean_name));
    let source_info = if path.exists() {
        let is_builtin = DEFAULT_TEMPLATES.iter().any(|dt| dt.name == clean_name);
        if is_builtin {
            format!("Local Override (.taskflow/templates/tasks/{}.toml)", clean_name)
        } else {
            format!("Local (.taskflow/templates/tasks/{}.toml)", clean_name)
        }
    } else {
        "Built-in".to_string()
    };

    println!("--------------------------------------------------");
    println!("TEMPLATE: {}", t.name);
    println!("Source:   {}", source_info);
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

pub fn init() -> Result<()> {
    let template_dir = Path::new(".taskflow/templates/tasks");
    if !template_dir.exists() {
        fs::create_dir_all(template_dir)?;
    }

    for dt in DEFAULT_TEMPLATES {
        let path = template_dir.join(dt.filename);
        if path.exists() {
            println!("Template {} is already present, skipping.", dt.filename);
        } else {
            fs::write(&path, dt.content)?;
            println!("Materialized template: {}", dt.filename);
        }
    }

    Ok(())
}
