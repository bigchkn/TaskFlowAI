use crate::model::Project;
use crate::storage::Storage;
use crate::roadmap;
use crate::model::TaskFragment;
use anyhow::Result;
use std::env;

pub fn run<S: Storage>(storage: &S, name: String) -> Result<()> {
    let project = Project {
        name,
        description: String::new(),
        version: "0.1.0".to_string(),
        milestones: Vec::new(),
        archived_milestones: Vec::new(),
        backlog_path: "roadmap/backlog.toml".to_string(),
    };
    storage.save_project(&project)?;
    storage.save_fragment(&project.backlog_path, &TaskFragment::default())?;
    let project_root = env::current_dir()?;
    roadmap::generate_roadmaps(storage, &project_root)?;
    println!("Initialized taskflow project");
    Ok(())
}
