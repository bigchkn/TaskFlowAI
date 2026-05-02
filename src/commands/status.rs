use crate::model::Status;
use crate::storage::Storage;
use crate::roadmap;
use anyhow::Result;
use std::env;
use chrono::Utc;

pub fn run<S: Storage>(storage: &S, task_id: String, new_status: String) -> Result<()> {
    let status = parse_status(&new_status)?;
    storage.update_task(&task_id, |task| {
        task.status = status;
        task.updated_at = Utc::now();
        if status == Status::Done {
            task.completed_at = Some(Utc::now());
        }
        Ok(())
    })?;
    let project_root = env::current_dir()?;
    roadmap::generate_roadmaps(storage, &project_root)?;
    println!("Updated task {} status to {:?}", task_id, status);
    Ok(())
}

pub fn parse_status(s: &str) -> Result<Status> {
    match s.to_lowercase().as_str() {
        "todo" => Ok(Status::Todo),
        "in-progress" | "progress" => Ok(Status::InProgress),
        "review" => Ok(Status::Review),
        "done" => Ok(Status::Done),
        "canceled" | "cancelled" => Ok(Status::Canceled),
        "backlog" => Ok(Status::Backlog),
        _ => Err(anyhow::anyhow!("Invalid status: {}", s)),
    }
}
