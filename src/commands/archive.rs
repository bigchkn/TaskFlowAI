use crate::roadmap;
use crate::storage::Storage;
use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::Path;

pub fn run<S: Storage>(storage: &S, storage_root: &Path, milestone_id: String) -> Result<()> {
    let mut project = storage.load_project()?;
    let index = match project
        .milestones
        .iter()
        .position(|m| m.id == milestone_id)
    {
        Some(idx) => idx,
        None => {
            eprintln!("Error: Milestone {} not found in active milestones", milestone_id);
            std::process::exit(1);
        }
    };

    let mut ms_meta = project.milestones.remove(index);

    // New path in archive/
    let old_path = ms_meta.path.clone();
    let new_path = format!("archive/{}.toml", milestone_id);

    // Move the file
    let old_full_path = storage_root.join(&old_path);
    let new_full_path = storage_root.join(&new_path);

    if let Some(parent) = new_full_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::rename(&old_full_path, &new_full_path).with_context(|| {
        format!(
            "Failed to move milestone file from {:?} to {:?}",
            old_full_path, new_full_path
        )
    })?;

    ms_meta.path = new_path;
    project.archived_milestones.push(ms_meta);

    storage.save_project(&project)?;
    let project_root = env::current_dir()?;
    roadmap::generate_roadmaps(storage, &project_root)?;
    println!("Archived milestone {}", milestone_id);
    Ok(())
}
