use crate::model::{Task, TaskType};
use crate::storage::Storage;
use crate::roadmap;
use anyhow::{Context, Result};
use std::env;

pub fn run<S: Storage>(
    storage: &S,
    title: String,
    task_type: Option<String>,
    milestone: Option<String>,
    parent: Option<String>,
) -> Result<()> {
    let project = storage.load_project()?;
    let t_type = match task_type.as_deref() {
        Some("bug") => TaskType::Bug,
        Some("chore") => TaskType::Chore,
        Some("research") => TaskType::Research,
        _ => TaskType::Feature,
    };

    let all_tasks = storage.load_all_tasks()?;
    let max_id = all_tasks
        .iter()
        .filter_map(|t| t.id.strip_prefix("TF-")?.parse::<usize>().ok())
        .max()
        .unwrap_or(0);
    let next_id = format!("TF-{}", max_id + 1);
    let mut task = Task::new(next_id, title, t_type);

    // Handle parent linkage
    let parent_ms_id = if let Some(ref parent_id) = parent {
        let parent_path = storage.find_task_path(parent_id)?;
        let parent_fragment = storage.load_fragment(&parent_path)?;
        let parent_task = parent_fragment
            .tasks
            .iter()
            .find(|t| t.id == *parent_id)
            .unwrap();

        task.parent_id = Some(parent_task.uid);

        let child_uid = task.uid;
        storage.update_task(parent_id, |p| {
            p.subtask_uids.push(child_uid);
            Ok(())
        })?;

        parent_task.milestone_id.clone()
    } else {
        None
    };

    let final_ms_id = milestone.or(parent_ms_id);

    if let Some(ms_id) = final_ms_id {
        let ms_meta = project
            .milestones
            .iter()
            .find(|m| m.id == ms_id)
            .with_context(|| format!("Milestone {} not found", ms_id))?;

        task.milestone_id = Some(ms_id.clone());
        let mut fragment = storage.load_fragment(&ms_meta.path)?;
        fragment.tasks.push(task);
        storage.save_fragment(&ms_meta.path, &fragment)?;
        let project_root = env::current_dir()?;
        roadmap::generate_roadmaps(storage, &project_root)?;
        println!(
            "Added task {} to milestone {} (Parent: {:?})",
            fragment.tasks.last().unwrap().id,
            ms_id,
            parent
        );
    } else {
        let mut fragment = storage.load_fragment(&project.backlog_path)?;
        fragment.tasks.push(task);
        storage.save_fragment(&project.backlog_path, &fragment)?;
        let project_root = env::current_dir()?;
        roadmap::generate_roadmaps(storage, &project_root)?;
        println!(
            "Added task {} to backlog (Parent: {:?})",
            fragment.tasks.last().unwrap().id,
            parent
        );
    }
    Ok(())
}
