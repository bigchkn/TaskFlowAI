use crate::model::Status;
use crate::storage::Storage;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn generate_roadmaps<S: Storage>(storage: &S, project_root: &Path) -> Result<()> {
    let project = storage.load_project()?;
    let all_tasks = storage.load_all_tasks()?;
    let uid_to_id: std::collections::HashMap<_, _> = all_tasks.iter().map(|t| (t.uid, t.id.clone())).collect();

    // 1. Generate ROADMAP_ACTIVE.md
    let mut active_md = format!("# Project Roadmap: {}\n\n", project.name);
    active_md.push_str(&format!("{}\n\n", project.description));

    active_md.push_str("## Active Milestones\n\n");
    if project.milestones.is_empty() {
        active_md.push_str("_No active milestones._\n\n");
    } else {
        for ms in &project.milestones {
            active_md.push_str(&format!("### {} ({})\n", ms.name, ms.id));
            if let Some(date) = ms.target_date {
                active_md.push_str(&format!("**Target Date:** {}\n", date.format("%Y-%m-%d")));
            }
            if ms.priority > 0 {
                active_md.push_str(&format!("**Priority:** {}\n", ms.priority));
            }
            active_md.push_str(&format!("**Status:** {:?}\n\n", ms.status));

            if !ms.designs.is_empty() {
                active_md.push_str("**Designs:**\n");
                for design in &ms.designs {
                    active_md.push_str(&format!("- [{:?}] {} (`{:?}`)\n", design.design_type, design.path, design.status));
                }
                active_md.push_str("\n");
            }

            let fragment = storage.load_fragment(&ms.path)?;
            for task in &fragment.tasks {
                let check = if task.status == Status::Done { "x" } else { " " };
                let indent = if task.parent_id.is_some() { "  " } else { "" };
                let parent_info = if let Some(p_uid) = task.parent_id {
                    let p_id = uid_to_id.get(&p_uid).map(|s| s.as_str()).unwrap_or("Unknown");
                    format!(" (Parent: {})", p_id)
                } else {
                    "".to_string()
                };

                active_md.push_str(&format!(
                    "{}- [{}] **{}**: {}{}(`{:?}`)\n",
                    indent, check, task.id, task.title, parent_info, task.status
                ));
                for design in &task.designs {
                    active_md.push_str(&format!("{}  - [{:?}] {} (`{:?}`)\n", indent, design.design_type, design.path, design.status));
                }
            }
            active_md.push_str("\n");
        }
    }

    active_md.push_str("## Backlog\n\n");
    let backlog = storage.load_fragment(&project.backlog_path)?;
    if backlog.tasks.is_empty() {
        active_md.push_str("_Backlog is empty._\n\n");
    } else {
        for task in &backlog.tasks {
            let check = if task.status == Status::Done { "x" } else { " " };
            let indent = if task.parent_id.is_some() { "  " } else { "" };
            let parent_info = if let Some(p_uid) = task.parent_id {
                let p_id = uid_to_id.get(&p_uid).map(|s| s.as_str()).unwrap_or("Unknown");
                format!(" (Parent: {})", p_id)
            } else {
                "".to_string()
            };

            active_md.push_str(&format!(
                "{}- [{}] **{}**: {}{}(`{:?}`)\n",
                indent, check, task.id, task.title, parent_info, task.status
            ));
            for design in &task.designs {
                active_md.push_str(&format!("{}  - [{:?}] {} (`{:?}`)\n", indent, design.design_type, design.path, design.status));
            }
        }
    }

    fs::write(project_root.join("ROADMAP_ACTIVE.md"), active_md)?;

    // 2. Generate ROADMAP_ARCHIVE.md
    let mut archive_md = format!("# Project Archive: {}\n\n", project.name);
    archive_md.push_str("## Archived Milestones\n\n");

    if project.archived_milestones.is_empty() {
        archive_md.push_str("_No archived milestones._\n\n");
    } else {
        for ms in &project.archived_milestones {
            archive_md.push_str(&format!("### {} ({})\n", ms.name, ms.id));
            if ms.priority > 0 {
                archive_md.push_str(&format!("**Priority:** {}\n", ms.priority));
            }
            archive_md.push_str(&format!("**Status:** {:?}\n\n", ms.status));

            if !ms.designs.is_empty() {
                archive_md.push_str("**Designs:**\n");
                for design in &ms.designs {
                    archive_md.push_str(&format!("- [{:?}] {} (`{:?}`)\n", design.design_type, design.path, design.status));
                }
                archive_md.push_str("\n");
            }

            let fragment = storage.load_fragment(&ms.path)?;
            for task in &fragment.tasks {
                let check = if task.status == Status::Done { "x" } else { " " };
                let indent = if task.parent_id.is_some() { "  " } else { "" };
                let parent_info = if let Some(p_uid) = task.parent_id {
                    let p_id = uid_to_id.get(&p_uid).map(|s| s.as_str()).unwrap_or("Unknown");
                    format!(" (Parent: {})", p_id)
                } else {
                    "".to_string()
                };

                archive_md.push_str(&format!(
                    "{}- [{}] **{}**: {}{}(`{:?}`)\n",
                    indent, check, task.id, task.title, parent_info, task.status
                ));
                for design in &task.designs {
                    archive_md.push_str(&format!("{}  - [{:?}] {} (`{:?}`)\n", indent, design.design_type, design.path, design.status));
                }
            }

            archive_md.push_str("\n");
        }
    }

    fs::write(project_root.join("ROADMAP_ARCHIVE.md"), archive_md)?;

    Ok(())
}
