use crate::model::Status;
use crate::roadmap;
use crate::storage::Storage;
use anyhow::Result;
use chrono::Utc;
use std::env;

pub fn start<S: Storage>(storage: &S, task_id: String, agent: Option<String>) -> Result<()> {
    storage.update_task(&task_id, |task| {
        task.status = Status::InProgress;
        task.execution.agent_id = agent;
        task.execution.start_time = Some(Utc::now());
        task.updated_at = Utc::now();
        Ok(())
    })?;
    let project_root = env::current_dir()?;
    roadmap::generate_roadmaps(storage, &project_root)?;
    println!("Started execution for task {}", task_id);
    Ok(())
}

pub fn complete<S: Storage>(
    storage: &S,
    task_id: String,
    outcome: Option<String>,
    log: Option<String>,
) -> Result<()> {
    storage.update_task(&task_id, |task| {
        task.status = Status::Done;
        task.execution.end_time = Some(Utc::now());
        task.execution.outcome = outcome;
        if let Some(l) = log {
            task.execution.logs.push(l);
        }
        task.completed_at = Some(Utc::now());
        task.updated_at = Utc::now();
        Ok(())
    })?;
    let project_root = env::current_dir()?;
    roadmap::generate_roadmaps(storage, &project_root)?;
    println!("Completed execution for task {}", task_id);
    Ok(())
}
