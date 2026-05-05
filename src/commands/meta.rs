use crate::roadmap;
use crate::storage::Storage;
use anyhow::Result;
use chrono::Utc;
use std::env;

pub fn set<S: Storage>(storage: &S, task_id: String, key: String, value: String) -> Result<()> {
    storage.update_task(&task_id, |task| {
        task.metadata.insert(key, value);
        task.updated_at = Utc::now();
        Ok(())
    })?;
    let project_root = env::current_dir()?;
    roadmap::generate_roadmaps(storage, &project_root)?;
    println!("Set metadata for task {}", task_id);
    Ok(())
}
