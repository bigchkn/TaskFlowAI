use crate::storage::Storage;
use anyhow::{Context, Result};

pub fn run<S: Storage>(storage: &S, task_id: String) -> Result<()> {
    let all_tasks = storage.load_all_tasks()?;
    let task = all_tasks
        .iter()
        .find(|t| t.id == task_id)
        .with_context(|| format!("Task {} not found", task_id))?;

    println!("--------------------------------------------------");
    println!("TASK: {} [{}]", task.title, task.id);
    println!("Type: {:?}", task.task_type);
    println!("Status: {:?}", task.status);
    println!("Priority: {}", task.priority);
    println!("--------------------------------------------------");

    if let Some(ms) = &task.milestone_id {
        println!("Milestone: {}", ms);
    } else {
        println!("Milestone: Backlog");
    }

    if let Some(parent) = task.parent_id {
        println!("Parent UID: {}", parent);
    }

    if !task.subtask_uids.is_empty() {
        println!("Subtask UIDs: {:?}", task.subtask_uids);
    }

    println!("--------------------------------------------------");
    println!(
        "Description: {}",
        if task.description.is_empty() {
            "None"
        } else {
            &task.description
        }
    );

    if !task.designs.is_empty() {
        println!("--------------------------------------------------");
        println!("DESIGNS:");
        for design in &task.designs {
            println!(
                "  - [{:?}] {} ({:?})",
                design.design_type, design.path, design.status
            );
        }
    }

    if !task.metadata.is_empty() {
        println!("--------------------------------------------------");
        println!("METADATA:");
        for (k, v) in &task.metadata {
            println!("  {}: {}", k, v);
        }
    }

    println!("--------------------------------------------------");
    println!("EXECUTION:");
    println!(
        "  Agent: {}",
        task.execution.agent_id.as_deref().unwrap_or("None")
    );
    println!("  Start: {:?}", task.execution.start_time);
    println!("  End:   {:?}", task.execution.end_time);
    println!(
        "  Outcome: {}",
        task.execution.outcome.as_deref().unwrap_or("Pending")
    );

    if !task.execution.logs.is_empty() {
        println!("  Logs:");
        for log in &task.execution.logs {
            println!("    - {}", log);
        }
    }
    println!("--------------------------------------------------");
    println!("Timestamps:");
    println!("  Created: {}", task.created_at);
    println!("  Updated: {}", task.updated_at);
    if let Some(comp) = task.completed_at {
        println!("  Completed: {}", comp);
    }
    println!("--------------------------------------------------");

    Ok(())
}
