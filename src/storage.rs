use crate::model::{Project, Task, TaskFragment};
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub trait Storage {
    fn load_project(&self) -> Result<Project>;
    fn save_project(&self, project: &Project) -> Result<()>;

    fn load_fragment(&self, relative_path: &str) -> Result<TaskFragment>;
    fn save_fragment(&self, relative_path: &str, fragment: &TaskFragment) -> Result<()>;

    fn load_active_tasks(&self) -> Result<Vec<Task>>;
    fn load_all_tasks(&self) -> Result<Vec<Task>>;

    fn update_task<F>(&self, task_id: &str, f: F) -> Result<Task>
    where
        F: FnOnce(&mut Task) -> Result<()>;
}

pub struct FileStorage {
    root: PathBuf, // Usually .taskflow/
}

impl FileStorage {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    fn roadmap_dir(&self) -> PathBuf {
        self.root.join("roadmap")
    }

    fn index_path(&self) -> PathBuf {
        self.roadmap_dir().join("index.toml")
    }
}

impl Storage for FileStorage {
    fn load_project(&self) -> Result<Project> {
        let path = self.index_path();
        if !path.exists() {
            return Ok(Project {
                name: "New Project".to_string(),
                description: String::new(),
                version: "0.1.0".to_string(),
                milestones: Vec::new(),
                archived_milestones: Vec::new(),
                backlog_path: "roadmap/backlog.toml".to_string(),
            });
        }
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read index at {:?}", path))?;
        let project: Project =
            toml::from_str(&content).with_context(|| "Failed to parse index TOML")?;
        Ok(project)
    }

    fn save_project(&self, project: &Project) -> Result<()> {
        let path = self.index_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(project)?;
        fs::write(&path, content)?;
        Ok(())
    }

    fn load_fragment(&self, relative_path: &str) -> Result<TaskFragment> {
        let path = self.root.join(relative_path);
        if !path.exists() {
            return Ok(TaskFragment::default());
        }
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read fragment at {:?}", path))?;
        let fragment: TaskFragment = toml::from_str(&content)
            .with_context(|| format!("Failed to parse fragment TOML at {:?}", path))?;
        Ok(fragment)
    }

    fn save_fragment(&self, relative_path: &str, fragment: &TaskFragment) -> Result<()> {
        let path = self.root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(fragment)?;
        fs::write(&path, content)?;
        Ok(())
    }

    fn load_active_tasks(&self) -> Result<Vec<Task>> {
        let project = self.load_project()?;
        let mut tasks = Vec::new();

        let backlog = self.load_fragment(&project.backlog_path)?;
        tasks.extend(backlog.tasks);

        for ms in &project.milestones {
            let fragment = self.load_fragment(&ms.path)?;
            tasks.extend(fragment.tasks);
        }

        Ok(tasks)
    }

    fn load_all_tasks(&self) -> Result<Vec<Task>> {
        let project = self.load_project()?;
        let mut all_tasks = self.load_active_tasks()?;

        for ms in &project.archived_milestones {
            let fragment = self.load_fragment(&ms.path)?;
            all_tasks.extend(fragment.tasks);
        }

        Ok(all_tasks)
    }

    fn update_task<F>(&self, task_id: &str, f: F) -> Result<Task>
    where
        F: FnOnce(&mut Task) -> Result<()>,
    {
        let project = self.load_project()?;

        // Check backlog
        let mut backlog = self.load_fragment(&project.backlog_path)?;
        if let Some(task) = backlog.tasks.iter_mut().find(|t| t.id == task_id) {
            f(task)?;
            let updated_task = task.clone();
            self.save_fragment(&project.backlog_path, &backlog)?;
            return Ok(updated_task);
        }

        // Check milestones
        for ms in &project.milestones {
            let mut fragment = self.load_fragment(&ms.path)?;
            if let Some(task) = fragment.tasks.iter_mut().find(|t| t.id == task_id) {
                f(task)?;
                let updated_task = task.clone();
                self.save_fragment(&ms.path, &fragment)?;
                return Ok(updated_task);
            }
        }

        Err(anyhow::anyhow!(
            "Task {} not found (or is archived)",
            task_id
        ))
    }
}
