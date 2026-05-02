use crate::storage::Storage;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn generate_roadmaps<S: Storage>(storage: &S, project_root: &Path) -> Result<()> {
    let project = storage.load_project()?;
    
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
            active_md.push_str(&format!("**Status:** {:?}\n\n", ms.status));
            
            let fragment = storage.load_fragment(&ms.path)?;
            for task in &fragment.tasks {
                active_md.push_str(&format!("- [ ] **{}**: {} (`{:?}`)\n", task.id, task.title, task.status));
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
            active_md.push_str(&format!("- [ ] **{}**: {} (`{:?}`)\n", task.id, task.title, task.status));
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
            archive_md.push_str(&format!("**Status:** {:?}\n\n", ms.status));
            
            let fragment = storage.load_fragment(&ms.path)?;
            for task in &fragment.tasks {
                archive_md.push_str(&format!("- [x] **{}**: {} (`{:?}`)\n", task.id, task.title, task.status));
            }
            archive_md.push_str("\n");
        }
    }
    
    fs::write(project_root.join("ROADMAP_ARCHIVE.md"), archive_md)?;
    
    Ok(())
}
