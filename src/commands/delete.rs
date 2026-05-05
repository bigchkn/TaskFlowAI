use crate::roadmap;
use crate::storage::Storage;
use anyhow::Result;
use std::env;

pub fn run<S: Storage>(storage: &S, task_id: String) -> Result<()> {
    let project = storage.load_project()?;

    let mut found = false;
    let mut fragment = storage.load_fragment(&project.backlog_path)?;
    if let Some(pos) = fragment.tasks.iter().position(|t| t.id == task_id) {
        fragment.tasks.remove(pos);
        storage.save_fragment(&project.backlog_path, &fragment)?;
        found = true;
    }

    if !found {
        for ms in &project.milestones {
            let mut fragment = storage.load_fragment(&ms.path)?;
            if let Some(pos) = fragment.tasks.iter().position(|t| t.id == task_id) {
                fragment.tasks.remove(pos);
                storage.save_fragment(&ms.path, &fragment)?;
                found = true;
                break;
            }
        }
    }

    if found {
        let project_root = env::current_dir()?;
        roadmap::generate_roadmaps(storage, &project_root)?;
        println!("Deleted task {}", task_id);
    } else {
        println!("Task {} not found", task_id);
    }
    Ok(())
}
