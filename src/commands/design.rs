use crate::model::{Design, DesignStatus, DesignType};
use crate::storage::Storage;
use crate::roadmap;
use anyhow::Result;
use std::env;
use std::path::PathBuf;
use std::fs;
use chrono::Utc;

pub fn init<S: Storage>(
    storage: &S,
    design_type: String,
    title: String,
    milestone: String,
    task: Option<String>,
) -> Result<()> {
    let d_type = parse_design_type(&design_type)?;
    let now = Utc::now();
    let project_root = env::current_dir()?;

    let mut relative_path = PathBuf::from("docs/designs");
    relative_path.push(&milestone);
    if let Some(ref t_id) = task {
        relative_path.push(t_id);
    }
    let filename = format!("{}-{}.md", design_type.to_lowercase(), title.to_lowercase().replace(" ", "-"));
    relative_path.push(filename);
    
    let full_path = project_root.join(&relative_path);
    if !full_path.exists() {
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let template_path = project_root.join(format!(".taskflow/templates/designs/{}.md", design_type.to_lowercase()));
        let mut content = if template_path.exists() {
            fs::read_to_string(template_path)?
        } else {
            format!("# Design: {}\n\nType: {}\n", title, design_type)
        };
        content = content.replace("{TITLE}", &title);
        fs::write(&full_path, content)?;
        println!("Scaffolded design document at {:?}", relative_path);
    }

    let design = Design {
        design_type: d_type,
        path: relative_path.to_string_lossy().to_string(),
        status: DesignStatus::Draft,
        created_at: now,
        updated_at: now,
    };

    if let Some(ref t_id) = task {
        storage.update_task(t_id, |t| {
            t.designs.push(design);
            t.updated_at = now;
            Ok(())
        })?;
    } else {
        storage.update_milestone(&milestone, |m| {
            m.designs.push(design);
            Ok(())
        })?;
    }
    roadmap::generate_roadmaps(storage, &project_root)?;
    println!("Registered design for {} {}", if task.is_some() { "task" } else { "milestone" }, if let Some(ref t_id) = task { t_id } else { &milestone });
    Ok(())
}

pub fn set_status<S: Storage>(
    storage: &S,
    path: String,
    status: String,
    milestone: String,
    task: Option<String>,
) -> Result<()> {
    let d_status = parse_design_status(&status)?;
    let now = Utc::now();
    if let Some(t_id) = task {
        storage.update_task(&t_id, |t| {
            if let Some(d) = t.designs.iter_mut().find(|d| d.path == path) {
                d.status = d_status;
                d.updated_at = now;
                t.updated_at = now;
                Ok(())
            } else {
                Err(anyhow::anyhow!("Design not found at path {} in task {}", path, t_id))
            }
        })?;
    } else {
        storage.update_milestone(&milestone, |m| {
            if let Some(d) = m.designs.iter_mut().find(|d| d.path == path) {
                d.status = d_status;
                d.updated_at = now;
                Ok(())
            } else {
                Err(anyhow::anyhow!("Design not found at path {} in milestone {}", path, milestone))
            }
        })?;
    }
    let project_root = env::current_dir()?;
    roadmap::generate_roadmaps(storage, &project_root)?;
    println!("Updated design status to {:?}", d_status);
    Ok(())
}

pub fn templates_list() -> Result<()> {
    println!("{:<15} | {:<20} | {:<40}", "DESIGN TYPE", "LOCAL TEMPLATE", "REQUIRED HEADERS COUNT");
    println!("{}", "-".repeat(80));

    for d_type in [DesignType::Hld, DesignType::Lld, DesignType::Rfc] {
        let type_str = match d_type {
            DesignType::Hld => "hld",
            DesignType::Lld => "lld",
            DesignType::Rfc => "rfc",
        };
        let template_path = format!(".taskflow/templates/designs/{}.md", type_str);
        let local_status = if std::path::Path::new(&template_path).exists() { "Present" } else { "Missing (Using Hardcoded)" };
        let req_count = d_type.required_headers().len();
        
        println!("{:<15} | {:<20} | {:<40}", type_str.to_uppercase(), local_status, req_count);
    }
    Ok(())
}

pub fn templates_show(design_type: &str) -> Result<()> {
    let d_type = parse_design_type(design_type)?;
    let type_str = design_type.to_lowercase();
    
    println!("--------------------------------------------------");
    println!("DESIGN TEMPLATE: {}", type_str.to_uppercase());
    println!("--------------------------------------------------");
    
    println!("REQUIRED HEADERS (Validation Enforced):");
    let headers = d_type.required_headers();
    if headers.is_empty() {
        println!("  (None)");
    } else {
        for header in headers {
            println!("  - {}", header);
        }
    }
    println!("--------------------------------------------------");
    
    let template_path = format!(".taskflow/templates/designs/{}.md", type_str);
    if std::path::Path::new(&template_path).exists() {
        println!("LOCAL SCAFFOLDING TEMPLATE (.taskflow/templates/designs/{}.md):", type_str);
        let content = fs::read_to_string(&template_path)?;
        println!("\n{}", content.trim());
    } else {
        println!("LOCAL SCAFFOLDING TEMPLATE: Missing (Will use minimal fallback)");
    }
    println!("--------------------------------------------------");

    Ok(())
}

fn parse_design_type(s: &str) -> Result<DesignType> {
    match s.to_lowercase().as_str() {
        "hld" => Ok(DesignType::Hld),
        "lld" => Ok(DesignType::Lld),
        "rfc" => Ok(DesignType::Rfc),
        _ => Err(anyhow::anyhow!("Invalid design type: {}", s)),
    }
}

fn parse_design_status(s: &str) -> Result<DesignStatus> {
    match s.to_lowercase().as_str() {
        "draft" => Ok(DesignStatus::Draft),
        "review" => Ok(DesignStatus::Review),
        "approved" => Ok(DesignStatus::Approved),
        "deprecated" => Ok(DesignStatus::Deprecated),
        _ => Err(anyhow::anyhow!("Invalid design status: {}", s)),
    }
}
