use crate::model::{Project, Task};
use anyhow::Result;
use std::path::PathBuf;

pub trait Storage {
    fn load_project(&self) -> Result<Project>;
    fn save_project(&self, project: &Project) -> Result<()>;
    fn save_task(&self, task: &Task) -> Result<()>;
    // ... more granular methods as needed
}

pub struct FileStorage {
    root: PathBuf,
}

impl FileStorage {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    fn project_path(&self) -> PathBuf {
        self.root.join("project.toml")
    }

    fn tasks_dir(&self) -> PathBuf {
        self.root.join("tasks")
    }
}

impl Storage for FileStorage {
    fn load_project(&self) -> Result<Project> {
        let path = self.project_path();
        if !path.exists() {
            return Ok(Project {
                name: "New Project".to_string(),
                description: String::new(),
                version: "0.1.0".to_string(),
                tasks: Vec::new(),
            });
        }
        let content = std::fs::read_to_string(path)?;
        let project: Project = toml::from_str(&content)?;
        // In a real implementation, we would also load tasks from the tasks_dir
        Ok(project)
    }

    fn save_project(&self, project: &Project) -> Result<()> {
        let path = self.project_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(project)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    fn save_task(&self, _task: &Task) -> Result<()> {
        // TODO: Implement individual task saving if we go that route
        Ok(())
    }
}
