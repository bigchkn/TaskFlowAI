use crate::model::DesignType;
use crate::storage::Storage;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn validate_task<S: Storage>(storage: &S, project_root: &Path, task_id: &str) -> Result<()> {
    let all_tasks = storage.load_all_tasks()?;
    let task = all_tasks
        .iter()
        .find(|t| t.id == task_id)
        .with_context(|| format!("Task {} not found", task_id))?;

    println!("Validating Task {}: {}", task.id, task.title);

    // 1. Check legacy lld_path
    if let Some(lld_path) = task.metadata.get("lld_path") {
        validate_design_file(project_root, lld_path, DesignType::Lld)?;
    }

    // 2. Check registered designs
    for design in &task.designs {
        validate_design_file(project_root, &design.path, design.design_type)?;
    }

    // 3. Check Task Template
    if let Some(template_name) = task.metadata.get("template") {
        if let Some(template) = crate::commands::templates::get_template(project_root, template_name) {
            println!("  - Validating against template: {}", template_name);

            // Check required metadata
            for (key, _) in &template.required_metadata {
                if let Some(value) = task.metadata.get(key) {
                    if value.trim().is_empty() {
                        return Err(anyhow::anyhow!(
                            "Task {} is missing required metadata field: '{}'",
                            task.id,
                            key
                        ));
                    }
                } else {
                    return Err(anyhow::anyhow!(
                        "Task {} is missing required metadata field: '{}'",
                        task.id,
                        key
                    ));
                }
            }

            // Check required designs
            for req in &template.required_designs {
                let has_design = task
                    .designs
                    .iter()
                    .any(|d| d.design_type == req.design_type);
                if !has_design {
                    return Err(anyhow::anyhow!(
                        "Task {} is missing required design type: {:?}",
                        task.id,
                        req.design_type
                    ));
                }
            }
            println!("  - Template validation passed.");
        }
    }

    Ok(())
}

pub fn validate_milestone<S: Storage>(
    storage: &S,
    project_root: &Path,
    milestone_id: &str,
) -> Result<()> {
    let project = storage.load_project()?;
    let ms = project
        .milestones
        .iter()
        .find(|m| m.id == milestone_id)
        .with_context(|| format!("Milestone {} not found", milestone_id))?;

    println!("Validating Milestone {}: {}", ms.id, ms.name);

    for design in &ms.designs {
        validate_design_file(project_root, &design.path, design.design_type)?;
    }

    Ok(())
}

fn validate_design_file(
    project_root: &Path,
    relative_path: &str,
    design_type: DesignType,
) -> Result<()> {
    let full_path = project_root.join(relative_path);
    println!("  - Checking {:?}: {}", design_type, relative_path);

    if !full_path.exists() {
        return Err(anyhow::anyhow!(
            "{:?} file not found at: {}",
            design_type,
            relative_path
        ));
    }

    let content = fs::read_to_string(&full_path)?;

    let required_headers = design_type.required_headers();

    for header in required_headers {
        if !content.contains(header) {
            return Err(anyhow::anyhow!(
                "{:?} is missing required header: '{}'",
                design_type,
                header
            ));
        }
    }
    println!("  - {:?} validation passed.", design_type);
    Ok(())
}
