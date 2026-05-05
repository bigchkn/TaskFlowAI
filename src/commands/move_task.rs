use crate::model::Status;
use crate::roadmap;
use crate::storage::Storage;
use anyhow::{Context, Result};
use std::env;

pub fn run<S: Storage>(storage: &S, task_id: String, milestone: String) -> Result<()> {
    let project = storage.load_project()?;
    let source_path = storage.find_task_path(&task_id)?;

    let target_milestone = project
        .milestones
        .iter()
        .find(|m| m.id == milestone || m.name == milestone)
        .context(format!("Milestone {} not found", milestone))?;

    let target_path = target_milestone.path.clone();

    if source_path == target_path {
        println!("Task {} is already in milestone {}", task_id, milestone);
        return Ok(());
    }

    let mut source_fragment = storage.load_fragment(&source_path)?;
    let mut target_fragment = storage.load_fragment(&target_path)?;

    let task_index = source_fragment
        .tasks
        .iter()
        .position(|t| t.id == task_id)
        .unwrap();
    let mut task = source_fragment.tasks.remove(task_index);

    if task.status == Status::Backlog {
        task.status = Status::Todo;
    }

    target_fragment.tasks.push(task);

    storage.save_fragment(&source_path, &source_fragment)?;
    storage.save_fragment(&target_path, &target_fragment)?;

    let project_root = env::current_dir()?;
    roadmap::generate_roadmaps(storage, &project_root)?;

    println!("Moved task {} to milestone {}", task_id, milestone);
    Ok(())
}
