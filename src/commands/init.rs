use crate::model::Project;
use crate::model::TaskFragment;
use crate::roadmap;
use crate::storage::Storage;
use anyhow::Result;
use indexmap::IndexMap;
use std::env;

pub fn run<S: Storage>(storage: &S, name: String) -> Result<()> {
    let project = Project {
        name,
        description: String::new(),
        version: "0.1.0".to_string(),
        milestones: Vec::new(),
        archived_milestones: Vec::new(),
        backlog_path: "roadmap/backlog.toml".to_string(),
        config: IndexMap::new(),
    };
    storage.save_project(&project)?;
    storage.save_fragment(&project.backlog_path, &TaskFragment::default())?;
    let project_root = env::current_dir()?;
    roadmap::generate_roadmaps(storage, &project_root)?;
    println!("Initialized taskflow project");
    Ok(())
}
