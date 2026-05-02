use crate::storage::Storage;
use crate::validation;
use anyhow::Result;
use std::env;

pub fn run<S: Storage>(storage: &S, task_id: String) -> Result<()> {
    let project_root = env::current_dir()?;
    if let Err(_) = validation::validate_task(storage, &project_root, &task_id) {
         validation::validate_milestone(storage, &project_root, &task_id)?;
    }
    Ok(())
}
