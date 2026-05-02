use crate::model::Task;
use crate::storage::Storage;
use crate::roadmap;
use anyhow::Result;
use std::env;
use std::fs;
use std::process::Command;

pub fn run<S: Storage>(storage: &S, task_id: String) -> Result<()> {
    let path = storage.find_task_path(&task_id)?;
    let mut fragment = storage.load_fragment(&path)?;

    let task_index = fragment
        .tasks
        .iter()
        .position(|t| t.id == task_id)
        .unwrap();
    let task = &fragment.tasks[task_index];

    // Create temporary file
    let temp_path = env::temp_dir().join(format!("{}.toml", task_id));
    let task_toml = toml::to_string_pretty(task)?;
    fs::write(&temp_path, task_toml)?;

    // Open editor
    let editor = env::var("EDITOR")
        .or_else(|_| env::var("VISUAL"))
        .unwrap_or_else(|_| "vi".to_string());

    let status = Command::new(&editor).arg(&temp_path).status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Editor exited with failure"));
    }

    // Read back and parse
    let edited_toml = fs::read_to_string(&temp_path)?;
    let updated_task: Task = toml::from_str(&edited_toml)?;

    if updated_task.id != task_id {
        return Err(anyhow::anyhow!(
            "Task ID change is not allowed during edit ({} -> {})",
            task_id,
            updated_task.id
        ));
    }

    // Update in fragment
    fragment.tasks[task_index] = updated_task;
    storage.save_fragment(&path, &fragment)?;

    // Cleanup
    let _ = fs::remove_file(&temp_path);

    // Sync
    let project_root = env::current_dir()?;
    roadmap::generate_roadmaps(storage, &project_root)?;

    println!("Updated task {}", task_id);
    Ok(())
}
