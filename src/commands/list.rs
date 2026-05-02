use crate::storage::Storage;
use anyhow::{Context, Result};

pub fn run<S: Storage>(storage: &S, milestone: Option<String>) -> Result<()> {
    let project = storage.load_project()?;
    if let Some(ms_id) = milestone {
        let ms_meta = project
            .milestones
            .iter()
            .find(|m| m.id == ms_id)
            .with_context(|| format!("Milestone {} not found", ms_id))?;
        let fragment = storage.load_fragment(&ms_meta.path)?;
        println!("Milestone: {} ({})", ms_meta.name, ms_meta.id);
        for task in &fragment.tasks {
            println!(
                "[{}] {} ({:?}) - {:?}",
                task.id, task.title, task.status, task.task_type
            );
        }
    } else {
        let all_tasks = storage.load_all_tasks()?;
        for task in all_tasks {
            let ms_info = task.milestone_id.as_deref().unwrap_or("Backlog");
            println!(
                "[{}] [{}] {} ({:?}) - {:?}",
                task.id, ms_info, task.title, task.status, task.task_type
            );
        }
    }
    Ok(())
}
