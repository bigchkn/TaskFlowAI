use crate::model::Status;
use crate::storage::Storage;
use anyhow::Result;

pub fn run<S: Storage>(storage: &S) -> Result<()> {
    let project = storage.load_project()?;
    let active_tasks = storage.load_active_tasks()?;
    let all_tasks = storage.load_all_tasks()?;

    println!("Dashboard: {}", project.name);
    println!("Active Milestones: {}", project.milestones.len());
    println!("Archived Milestones: {}", project.archived_milestones.len());
    println!("---");
    println!("Active Tasks: {}", active_tasks.len());
    let active_done = active_tasks
        .iter()
        .filter(|t| t.status == Status::Done)
        .count();
    println!(
        "Active Progress: {}/{} completed",
        active_done,
        active_tasks.len()
    );
    println!("---");
    println!("Total Project Tasks: {}", all_tasks.len());
    Ok(())
}
