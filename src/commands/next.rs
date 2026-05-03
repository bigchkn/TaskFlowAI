use crate::model::Status;
use crate::storage::Storage;
use anyhow::Result;

pub fn run<S: Storage>(storage: &S) -> Result<()> {
    let project = storage.load_project()?;
    let mut next_task = None;

    // 1. Check Milestones (sorted by priority)
    let mut sorted_milestones = project.milestones.clone();
    sorted_milestones.sort_by(|a, b| b.priority.cmp(&a.priority));

    for ms in &sorted_milestones {
        let fragment = storage.load_fragment(&ms.path)?;

        if let Some(task) = fragment.tasks.iter().find(|t| t.status == Status::InProgress) {
            next_task = Some((task.clone(), ms.name.clone()));
            break;
        }

        if let Some(task) = fragment.tasks.iter().find(|t| t.status == Status::Todo) {
            next_task = Some((task.clone(), ms.name.clone()));
            break;
        }

        if let Some(task) = fragment.tasks.iter().find(|t| t.status == Status::Backlog) {
            next_task = Some((task.clone(), ms.name.clone()));
            break;
        }
    }

    // 2. Check Global Backlog
    if next_task.is_none() {
        let backlog = storage.load_fragment(&project.backlog_path)?;
        if let Some(task) = backlog.tasks.iter().find(|t| t.status != Status::Done && t.status != Status::Canceled) {
            next_task = Some((task.clone(), "Global Backlog".to_string()));
        }
    }

    if let Some((task, ms_name)) = next_task {
        println!(">>> Next Task: {} - {}", task.id, task.title);
        println!("Milestone: {}", ms_name);
        println!("Status:    {:?}", task.status);
        println!("Priority:  {}", task.priority);

        if !task.designs.is_empty() {
            println!("\nDesigns:");
            for design in &task.designs {
                println!("  - [{:?}] {} (`{:?}`)", design.design_type, design.path, design.status);
            }
        }

        if !task.metadata.is_empty() {
            println!("\nMetadata:");
            for (k, v) in &task.metadata {
                println!("  {}: {}", k, v);
            }
        }

        println!("\nSuggested Action:");
        match task.status {
            Status::Backlog | Status::Todo => {
                println!("  taskflow-ai execute start {}", task.id);
            }
            Status::InProgress => {
                println!("  taskflow-ai execute complete {}", task.id);
            }
            Status::Review => {
                println!("  taskflow-ai status {} done", task.id);
            }
            _ => {}
        }
    } else {
        println!("No pending tasks found. Project complete or backlog empty!");
    }
    Ok(())
}
