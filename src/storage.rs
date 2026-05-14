use crate::model::{Project, Task, TaskFragment};
use anyhow::{Context, Result};
use indexmap::IndexMap;
use std::fs::{self, File};
use std::path::PathBuf;

pub trait Storage {
    /// Acquire an exclusive lock on the project to prevent concurrent modifications.
    /// The lock is released when the returned guard is dropped.
    fn lock_exclusive(&self) -> Result<Box<dyn std::any::Any>>;

    fn load_project(&self) -> Result<Project>;
    fn save_project(&self, project: &Project) -> Result<()>;

    fn load_fragment(&self, relative_path: &str) -> Result<TaskFragment>;
    fn save_fragment(&self, relative_path: &str, fragment: &TaskFragment) -> Result<()>;

    fn load_active_tasks(&self) -> Result<Vec<Task>>;
    fn load_all_tasks(&self) -> Result<Vec<Task>>;

    fn update_task<F>(&self, task_id: &str, f: F) -> Result<Task>
    where
        F: FnOnce(&mut Task) -> Result<()>;

    fn update_milestone<F>(&self, milestone_id: &str, f: F) -> Result<()>
    where
        F: FnOnce(&mut crate::model::MilestoneMetadata) -> Result<()>;

    fn find_task_path(&self, task_id: &str) -> Result<String>;
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
    fn lock_exclusive(&self) -> Result<Box<dyn std::any::Any>> {
        let path = self.index_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .with_context(|| format!("Failed to open index for locking at {:?}", path))?;

        let lock = Box::new(fd_lock::RwLock::new(file));
        
        // We leak the lock to get a 'static reference, allowing us to store the guard
        let lock_static: &'static mut fd_lock::RwLock<File> = Box::leak(lock);
        let guard = lock_static.write().map_err(|e| anyhow::anyhow!("Failed to acquire project lock: {}", e))?;
        
        Ok(Box::new(guard))
    }

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
                config: IndexMap::new(),
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

    fn update_milestone<F>(&self, milestone_id: &str, f: F) -> Result<()>
    where
        F: FnOnce(&mut crate::model::MilestoneMetadata) -> Result<()>,
    {
        let mut project = self.load_project()?;
        if let Some(ms) = project.milestones.iter_mut().find(|m| m.id == milestone_id) {
            f(ms)?;
            self.save_project(&project)?;
            return Ok(());
        }
        Err(anyhow::anyhow!("Milestone {} not found", milestone_id))
    }

    fn find_task_path(&self, task_id: &str) -> Result<String> {
        let project = self.load_project()?;

        // Check backlog
        let backlog = self.load_fragment(&project.backlog_path)?;
        if backlog.tasks.iter().any(|t| t.id == task_id) {
            return Ok(project.backlog_path);
        }

        // Check milestones
        for ms in &project.milestones {
            let fragment = self.load_fragment(&ms.path)?;
            if fragment.tasks.iter().any(|t| t.id == task_id) {
                return Ok(ms.path.clone());
            }
        }

        Err(anyhow::anyhow!("Task {} not found", task_id))
    }
}
