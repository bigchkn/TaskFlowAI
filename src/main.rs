mod model;
mod storage;

use clap::{Parser, Subcommand};
use crate::model::{Task, TaskType};
use crate::storage::{FileStorage, Storage};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "taskflow")]
#[command(about = "Task Management system designed for AI collaboration", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new taskflow project
    Init {
        #[arg(default_value = "New Project")]
        name: String,
    },
    /// Add a new task
    Add {
        title: String,
        #[arg(short, long)]
        task_type: Option<String>,
    },
    /// List all tasks
    List,
    /// Show project status
    Status,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let storage_root = std::env::current_dir()?.join(".taskflow");
    let storage = FileStorage::new(storage_root);

    match cli.command {
        Commands::Init { name } => {
            let project = model::Project {
                name,
                description: String::new(),
                version: "0.1.0".to_string(),
                tasks: Vec::new(),
            };
            storage.save_project(&project)?;
            println!("Initialized taskflow project");
        }
        Commands::Add { title, task_type } => {
            let mut project = storage.load_project()?;
            let t_type = match task_type.as_deref() {
                Some("bug") => TaskType::Bug,
                Some("chore") => TaskType::Chore,
                Some("research") => TaskType::Research,
                Some("milestone") => TaskType::Milestone,
                _ => TaskType::Feature,
            };
            
            let next_id = format!("TF-{}", project.tasks.len() + 1);
            let task = Task::new(next_id, title, t_type);
            project.tasks.push(task);
            storage.save_project(&project)?;
            println!("Added task");
        }
        Commands::List => {
            let project = storage.load_project()?;
            println!("Project: {}", project.name);
            for task in &project.tasks {
                println!("[{}] {} ({:?}) - {:?}", task.id, task.title, task.status, task.task_type);
            }
        }
        Commands::Status => {
            let project = storage.load_project()?;
            println!("Project: {}", project.name);
            println!("Total tasks: {}", project.tasks.len());
        }
    }

    Ok(())
}
