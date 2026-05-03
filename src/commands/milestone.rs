use crate::model::{MilestoneMetadata, TaskFragment};
use crate::storage::Storage;
use crate::roadmap;
use crate::model::Status;
use anyhow::Result;
use std::env;

pub fn create<S: Storage>(storage: &S, id: String, name: String, priority: u8) -> Result<()> {
    let mut project = storage.load_project()?;
    if project.milestones.iter().any(|m| m.id == id) {
        return Err(anyhow::anyhow!("Milestone {} already exists", id));
    }
    let path = format!("roadmap/{}.toml", id);
    project.milestones.push(MilestoneMetadata {
        id: id.clone(),
        name,
        description: String::new(),
        target_date: None,
        status: Status::Todo,
        path: path.clone(),
        priority,
        designs: Vec::new(),
    });
    storage.save_project(&project)?;
    storage.save_fragment(&path, &TaskFragment::default())?;
    let project_root = env::current_dir()?;
    roadmap::generate_roadmaps(storage, &project_root)?;
    println!("Created milestone {} with priority {}", id, priority);
    Ok(())
}

pub fn edit<S: Storage>(storage: &S, id: String, name: Option<String>, priority: Option<u8>) -> Result<()> {
    let mut project = storage.load_project()?;
    let milestone = project.milestones.iter_mut().find(|m| m.id == id)
        .ok_or_else(|| anyhow::anyhow!("Milestone {} not found", id))?;

    if let Some(n) = name {
        milestone.name = n;
    }
    if let Some(p) = priority {
        milestone.priority = p;
    }

    storage.save_project(&project)?;
    let project_root = env::current_dir()?;
    roadmap::generate_roadmaps(storage, &project_root)?;
    println!("Updated milestone {}", id);
    Ok(())
}

pub fn list<S: Storage>(storage: &S) -> Result<()> {
    let project = storage.load_project()?;
    for ms in &project.milestones {
        println!("{} - {} (priority: {}, path: {})", ms.id, ms.name, ms.priority, ms.path);
    }
    Ok(())
}
