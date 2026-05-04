use crate::storage::Storage;
use crate::validation;
use anyhow::Result;
use std::env;

pub fn run<S: Storage>(storage: &S, task_id: String) -> Result<()> {
    let project_root = env::current_dir()?;
    
    // Check if it's a task first
    let all_tasks = storage.load_all_tasks()?;
    if all_tasks.iter().any(|t| t.id == task_id) {
        validation::validate_task(storage, &project_root, &task_id)?;
        return Ok(());
    }

    // Otherwise try as a milestone
    let project = storage.load_project()?;
    if project.milestones.iter().any(|m| m.id == task_id) {
        validation::validate_milestone(storage, &project_root, &task_id)?;
        return Ok(());
    }

    Err(anyhow::anyhow!("Entity {} not found as task or milestone", task_id))
}
