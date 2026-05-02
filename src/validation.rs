use crate::storage::Storage;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn validate_task<S: Storage>(storage: &S, project_root: &Path, task_id: &str) -> Result<()> {
    // 1. Load the task
    // We can't directly use storage.update_task because we don't want to mutate it here.
    // We'll use load_all_tasks to find it.
    let all_tasks = storage.load_all_tasks()?;
    let task = all_tasks
        .iter()
        .find(|t| t.id == task_id)
        .with_context(|| format!("Task {} not found", task_id))?;

    println!("Validating Task {}: {}", task.id, task.title);

    // 2. Check for linked design documents
    if let Some(lld_path) = task.metadata.get("lld_path") {
        let full_lld_path = project_root.join(lld_path);
        println!("  - Checking LLD: {}", lld_path);

        if !full_lld_path.exists() {
            return Err(anyhow::anyhow!("LLD file not found at: {}", lld_path));
        }

        let content = fs::read_to_string(&full_lld_path)?;

        // Basic template validation: check for required headers
        let required_headers = vec![
            "# Low-Level Design:",
            "## 1. Objective",
            "## 2. Architecture",
            "## 3. Implementation Details",
            "## 4. Verification Plan",
        ];

        for header in required_headers {
            if !content.contains(header) {
                return Err(anyhow::anyhow!(
                    "LLD is missing required header: '{}'",
                    header
                ));
            }
        }
        println!("  - LLD validation passed.");
    } else {
        println!("  - No LLD linked (lld_path metadata not set).");
    }

    // 3. check for other metadata requirements if we have task templates (TBD)

    Ok(())
}
