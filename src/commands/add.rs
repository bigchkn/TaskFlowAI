use crate::roadmap;
use crate::storage::Storage;
use anyhow::{Context, Result};
use std::env;

pub fn run<S: Storage>(
    storage: &S,
    title: String,
    task_type: Option<String>,
    milestone: Option<String>,
    parent: Option<String>,
    template_name: Option<String>,
) -> Result<()> {
    let project = storage.load_project()?;
    let t_type = match task_type.as_deref() {
        Some("bug") => crate::model::TaskType::Bug,
        Some("chore") => crate::model::TaskType::Chore,
        Some("research") => crate::model::TaskType::Research,
        _ => crate::model::TaskType::Feature,
    };

    let all_tasks = storage.load_all_tasks()?;
    let max_id = all_tasks
        .iter()
        .filter_map(|t| t.id.strip_prefix("TF-")?.parse::<usize>().ok())
        .max()
        .unwrap_or(0);
    let next_id = format!("TF-{}", max_id + 1);
    let mut task = crate::model::Task::new(next_id.clone(), title, t_type);

    // Load template if provided or configured
    let t_name = template_name.or_else(|| {
        project
            .config
            .get("default_template")
            .filter(|v| !v.trim().is_empty())
            .cloned()
    });

    // Check if templates are forced
    if t_name.is_none()
        && parent.is_none()
        && project
            .config
            .get("force_templates")
            .map(|v| v == "true")
            .unwrap_or(false)
    {
        return Err(anyhow::anyhow!(
            "Project configuration 'force_templates' is enabled. You must provide a template with --template or set a 'default_template'."
        ));
    }

    let mut template = None;
    if let Some(name) = t_name {
        let template_path = std::env::current_dir()?
            .join(".taskflow/templates/tasks")
            .join(format!("{}.toml", name));
        if template_path.exists() {
            let content = std::fs::read_to_string(template_path)?;
            let t: crate::model::TaskTemplate = toml::from_str(&content)?;
            task.metadata.insert("template".to_string(), name.clone());
            for (k, _v) in &t.required_metadata {
                task.metadata.insert(k.clone(), String::new());
            }
            template = Some(t);
        }
    }

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

    // Scaffold subtasks from template
    if let Some(t) = template {
        for sub in t.default_subtasks {
            let sub_type = Some(match sub.task_type {
                crate::model::TaskType::Bug => "bug".to_string(),
                crate::model::TaskType::Chore => "chore".to_string(),
                crate::model::TaskType::Research => "research".to_string(),
                _ => "feature".to_string(),
            });
            run(
                storage,
                sub.title,
                sub_type,
                None, // Inherit milestone from parent (handled in run)
                Some(next_id.clone()),
                None, // Don't apply templates recursively to subtasks for now
            )?;
        }
    }
    Ok(())
}
