mod model;
mod roadmap;
mod storage;
mod validation;

use crate::model::{
    Design, DesignStatus, DesignType, MilestoneMetadata, Project, Status, Task, TaskFragment,
    TaskType,
};
use crate::storage::{FileStorage, Storage};
use anyhow::{Context, Result};
use chrono::Utc;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

// ... (Cli and Commands enums)

#[derive(Parser)]
#[command(name = "taskflow-ai")]
#[command(about = "Task Management system designed for AI collaboration", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
        #[arg(short, long)]
        milestone: Option<String>,
    },
    /// List all tasks
    List {
        #[arg(short, long)]
        milestone: Option<String>,
    },
    /// Milestone management
    Milestone {
        #[command(subcommand)]
        command: MilestoneCommands,
    },
    /// Update task status
    Status { task_id: String, new_status: String },
    /// Execution tracking
    Execute {
        #[command(subcommand)]
        command: ExecuteCommands,
    },
    /// Metadata management
    Meta {
        #[command(subcommand)]
        command: MetaCommands,
    },
    /// Design document management
    Design {
        #[command(subcommand)]
        command: DesignCommands,
    },
    /// Validate a task against its template requirements
    Validate { task_id: String },

    /// Suggest the next task to work on
    Next,

    /// Archive a completed milestone
    Archive { milestone_id: String },
    /// Sync and regenerate Markdown roadmap files
    Sync,
    /// Show project dashboard
    Dashboard,
}

#[derive(Subcommand)]
enum MilestoneCommands {
    /// Create a new milestone
    Create {
        id: String, // e.g., M1
        name: String,
    },
    /// List all milestones
    List,
}

#[derive(Subcommand)]
enum ExecuteCommands {
    /// Start task execution
    Start {
        task_id: String,
        #[arg(short, long)]
        agent: Option<String>,
    },
    /// Complete task execution
    Complete {
        task_id: String,
        #[arg(short, long)]
        outcome: Option<String>,
        #[arg(short, long)]
        log: Option<String>,
    },
}

#[derive(Subcommand)]
enum DesignCommands {
    /// Initialize a new design document
    Init {
        design_type: String, // hld or lld
        title: String,
        #[arg(short, long)]
        milestone: String,
        #[arg(short, long)]
        task: Option<String>,
    },
    /// Update the status of a design document
    Status {
        path: String,
        status: String,
        #[arg(short, long)]
        milestone: String,
        #[arg(short, long)]
        task: Option<String>,
    },
}

#[derive(Subcommand)]
enum MetaCommands {
    /// Set a metadata key-value pair
    Set {
        task_id: String,
        key: String,
        value: String,
    },
}

fn parse_design_type(s: &str) -> Result<DesignType> {
    match s.to_lowercase().as_str() {
        "hld" => Ok(DesignType::Hld),
        "lld" => Ok(DesignType::Lld),
        "rfc" => Ok(DesignType::Rfc),
        _ => Err(anyhow::anyhow!("Unknown design type: {}", s)),
    }
}

fn parse_design_status(s: &str) -> Result<DesignStatus> {
    match s.to_lowercase().as_str() {
        "draft" => Ok(DesignStatus::Draft),
        "review" => Ok(DesignStatus::Review),
        "approved" => Ok(DesignStatus::Approved),
        "deprecated" => Ok(DesignStatus::Deprecated),
        _ => Err(anyhow::anyhow!("Unknown design status: {}", s)),
    }
}

fn parse_status(s: &str) -> Result<Status> {
    match s.to_lowercase().as_str() {
        "backlog" => Ok(Status::Backlog),
        "todo" => Ok(Status::Todo),
        "inprogress" | "in-progress" => Ok(Status::InProgress),
        "review" => Ok(Status::Review),
        "done" => Ok(Status::Done),
        "blocked" => Ok(Status::Blocked),
        "canceled" | "cancelled" => Ok(Status::Canceled),
        _ => Err(anyhow::anyhow!("Unknown status: {}", s)),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let storage_root = std::env::current_dir()?.join(".taskflow");
    let storage = FileStorage::new(storage_root.clone());

    match cli.command {
        Commands::Init { name } => {
            let project = Project {
                name,
                description: String::new(),
                version: "0.1.0".to_string(),
                milestones: Vec::new(),
                archived_milestones: Vec::new(),
                backlog_path: "roadmap/backlog.toml".to_string(),
            };
            storage.save_project(&project)?;
            storage.save_fragment(&project.backlog_path, &TaskFragment::default())?;
            let project_root = std::env::current_dir()?;
            roadmap::generate_roadmaps(&storage, &project_root)?;
            println!("Initialized taskflow project");
        }
        Commands::Add {
            title,
            task_type,
            milestone,
        } => {
            let project = storage.load_project()?;
            let t_type = match task_type.as_deref() {
                Some("bug") => TaskType::Bug,
                Some("chore") => TaskType::Chore,
                Some("research") => TaskType::Research,
                _ => TaskType::Feature,
            };

            let all_tasks = storage.load_all_tasks()?;
            let next_id = format!("TF-{}", all_tasks.len() + 1);
            let mut task = Task::new(next_id, title, t_type);

            if let Some(ms_id) = milestone {
                let ms_meta = project
                    .milestones
                    .iter()
                    .find(|m| m.id == ms_id)
                    .with_context(|| format!("Milestone {} not found", ms_id))?;

                task.milestone_id = Some(ms_id.clone());
                let mut fragment = storage.load_fragment(&ms_meta.path)?;
                fragment.tasks.push(task);
                storage.save_fragment(&ms_meta.path, &fragment)?;
                let project_root = std::env::current_dir()?;
                roadmap::generate_roadmaps(&storage, &project_root)?;
                println!(
                    "Added task {} to milestone {}",
                    fragment.tasks.last().unwrap().id,
                    ms_id
                );
            } else {
                let mut fragment = storage.load_fragment(&project.backlog_path)?;
                fragment.tasks.push(task);
                storage.save_fragment(&project.backlog_path, &fragment)?;
                let project_root = std::env::current_dir()?;
                roadmap::generate_roadmaps(&storage, &project_root)?;
                println!(
                    "Added task {} to backlog",
                    fragment.tasks.last().unwrap().id
                );
            }
        }
        Commands::List { milestone } => {
            let project = storage.load_project()?;
            if let Some(ms_id) = milestone {
                let ms_meta = project
                    .milestones
                    .iter()
                    .find(|m| m.id == ms_id)
                    .with_context(|| format!("Milestone {} not found", ms_id))?;
                let fragment = storage.load_fragment(&ms_meta.path)?;
                println!("Milestone: {} ({})", ms_meta.name, ms_meta.id);
                for task in &fragment.tasks {
                    println!(
                        "[{}] {} ({:?}) - {:?}",
                        task.id, task.title, task.status, task.task_type
                    );
                }
            } else {
                let all_tasks = storage.load_all_tasks()?;
                for task in all_tasks {
                    let ms_info = task.milestone_id.as_deref().unwrap_or("Backlog");
                    println!(
                        "[{}] [{}] {} ({:?}) - {:?}",
                        task.id, ms_info, task.title, task.status, task.task_type
                    );
                }
            }
        }
        Commands::Status {
            task_id,
            new_status,
        } => {
            let status = parse_status(&new_status)?;
            storage.update_task(&task_id, |task| {
                task.status = status;
                task.updated_at = Utc::now();
                if status == Status::Done {
                    task.completed_at = Some(Utc::now());
                }
                Ok(())
            })?;
            let project_root = std::env::current_dir()?;
            roadmap::generate_roadmaps(&storage, &project_root)?;
            println!("Updated task {} status to {:?}", task_id, status);
        }
        Commands::Archive { milestone_id } => {
            let mut project = storage.load_project()?;
            let index = project
                .milestones
                .iter()
                .position(|m| m.id == milestone_id)
                .with_context(|| {
                    format!("Milestone {} not found in active milestones", milestone_id)
                })?;

            let mut ms_meta = project.milestones.remove(index);

            // New path in archive/
            let old_path = ms_meta.path.clone();
            let new_path = format!("archive/{}.toml", milestone_id);

            // Move the file
            let old_full_path = storage_root.join(&old_path);
            let new_full_path = storage_root.join(&new_path);

            if let Some(parent) = new_full_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::rename(&old_full_path, &new_full_path).with_context(|| {
                format!(
                    "Failed to move milestone file from {:?} to {:?}",
                    old_full_path, new_full_path
                )
            })?;

            ms_meta.path = new_path;
            project.archived_milestones.push(ms_meta);

            storage.save_project(&project)?;
            let project_root = std::env::current_dir()?;
            roadmap::generate_roadmaps(&storage, &project_root)?;
            println!("Archived milestone {}", milestone_id);
        }
        Commands::Sync => {
            let project_root = std::env::current_dir()?;
            roadmap::generate_roadmaps(&storage, &project_root)?;
            println!("Roadmap files synced");
        }
        Commands::Dashboard => {
            let project = storage.load_project()?;
            let active_tasks = storage.load_active_tasks()?;
            let all_tasks = storage.load_all_tasks()?;

            println!("Dashboard: {}", project.name);
            println!("Active Milestones: {}", project.milestones.len());
            println!("Archived Milestones: {}", project.archived_milestones.len());
            println!("---");
            println!("Active Tasks: {}", active_tasks.len());
            let active_done = active_tasks
                .iter()
                .filter(|t| t.status == Status::Done)
                .count();
            println!(
                "Active Progress: {}/{} completed",
                active_done,
                active_tasks.len()
            );
            println!("---");
            println!("Total Project Tasks: {}", all_tasks.len());
        }
        Commands::Milestone { command } => match command {
            MilestoneCommands::Create { id, name } => {
                let mut project = storage.load_project()?;
                if project.milestones.iter().any(|m| m.id == id) {
                    return Err(anyhow::anyhow!("Milestone {} already exists", id));
                }
                let path = format!("roadmap/{}.toml", id);
                project.milestones.push(MilestoneMetadata {
                    id: id.clone(),
                    name,
                    description: String::new(),
                    target_date: None,
                    status: Status::Todo,
                    path: path.clone(),
                    designs: Vec::new(),
                });
                storage.save_project(&project)?;
                storage.save_fragment(&path, &TaskFragment::default())?;
                let project_root = std::env::current_dir()?;
                roadmap::generate_roadmaps(&storage, &project_root)?;
                println!("Created milestone {}", id);
            }
            MilestoneCommands::List => {
                let project = storage.load_project()?;
                for ms in &project.milestones {
                    println!("{} - {} ({})", ms.id, ms.name, ms.path);
                }
            }
        },
        Commands::Execute { command } => match command {
            ExecuteCommands::Start { task_id, agent } => {
                storage.update_task(&task_id, |task| {
                    task.status = Status::InProgress;
                    task.execution.agent_id = agent;
                    task.execution.start_time = Some(Utc::now());
                    task.updated_at = Utc::now();
                    Ok(())
                })?;
                let project_root = std::env::current_dir()?;
                roadmap::generate_roadmaps(&storage, &project_root)?;
                println!("Started execution for task {}", task_id);
            }
            ExecuteCommands::Complete {
                task_id,
                outcome,
                log,
            } => {
                storage.update_task(&task_id, |task| {
                    task.status = Status::Done;
                    task.execution.end_time = Some(Utc::now());
                    task.execution.outcome = outcome;
                    if let Some(l) = log {
                        task.execution.logs.push(l);
                    }
                    task.completed_at = Some(Utc::now());
                    task.updated_at = Utc::now();
                    Ok(())
                })?;
                let project_root = std::env::current_dir()?;
                roadmap::generate_roadmaps(&storage, &project_root)?;
                println!("Completed execution for task {}", task_id);
            }
        },
        Commands::Meta { command } => match command {
            MetaCommands::Set {
                task_id,
                key,
                value,
            } => {
                storage.update_task(&task_id, |task| {
                    task.metadata.insert(key, value);
                    task.updated_at = Utc::now();
                    Ok(())
                })?;
                let project_root = std::env::current_dir()?;
                roadmap::generate_roadmaps(&storage, &project_root)?;
                println!("Set metadata for task {}", task_id);
            }
        },
        Commands::Design { command } => match command {
            DesignCommands::Init {
                design_type,
                title,
                milestone,
                task,
            } => {
                let d_type = parse_design_type(&design_type)?;
                let now = Utc::now();
                let project_root = std::env::current_dir()?;

                // Determine path: docs/designs/<milestone>[/<task>]/<type>-<title>.md
                let mut relative_path = PathBuf::from("docs/designs");
                relative_path.push(&milestone);
                if let Some(ref t_id) = task {
                    relative_path.push(t_id);
                }
                let filename = format!("{}-{}.md", design_type.to_lowercase(), title.to_lowercase().replace(" ", "-"));
                relative_path.push(filename);
                
                let full_path = project_root.join(&relative_path);
                if !full_path.exists() {
                    if let Some(parent) = full_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    // Try to load template
                    let template_path = project_root.join(format!(".taskflow/templates/designs/{}.md", design_type.to_lowercase()));
                    let mut content = if template_path.exists() {
                        std::fs::read_to_string(template_path)?
                    } else {
                        format!("# Design: {}\n\nType: {}\n", title, design_type)
                    };
                    content = content.replace("{TITLE}", &title);
                    std::fs::write(&full_path, content)?;
                    println!("Scaffolded design document at {:?}", relative_path);
                }

                let design = Design {
                    design_type: d_type,
                    path: relative_path.to_string_lossy().to_string(),
                    status: DesignStatus::Draft,
                    created_at: now,
                    updated_at: now,
                };

                if let Some(ref t_id) = task {
                    storage.update_task(t_id, |t| {
                        t.designs.push(design);
                        t.updated_at = now;
                        Ok(())
                    })?;
                } else {
                    storage.update_milestone(&milestone, |m| {
                        m.designs.push(design);
                        Ok(())
                    })?;
                }
                let project_root = std::env::current_dir()?;
                roadmap::generate_roadmaps(&storage, &project_root)?;
                println!("Registered design for {} {}", if task.is_some() { "task" } else { "milestone" }, if let Some(ref t_id) = task { t_id } else { &milestone });
            }
            DesignCommands::Status {
                path,
                status,
                milestone,
                task,
            } => {
                let d_status = parse_design_status(&status)?;
                let now = Utc::now();
                if let Some(t_id) = task {
                    storage.update_task(&t_id, |t| {
                        if let Some(d) = t.designs.iter_mut().find(|d| d.path == path) {
                            d.status = d_status;
                            d.updated_at = now;
                            t.updated_at = now;
                            Ok(())
                        } else {
                            Err(anyhow::anyhow!("Design not found at path {} in task {}", path, t_id))
                        }
                    })?;
                } else {
                    storage.update_milestone(&milestone, |m| {
                        if let Some(d) = m.designs.iter_mut().find(|d| d.path == path) {
                            d.status = d_status;
                            d.updated_at = now;
                            Ok(())
                        } else {
                            Err(anyhow::anyhow!("Design not found at path {} in milestone {}", path, milestone))
                        }
                    })?;
                }
                let project_root = std::env::current_dir()?;
                roadmap::generate_roadmaps(&storage, &project_root)?;
                println!("Updated design status to {:?}", d_status);
            }
        },
        Commands::Validate { task_id } => {
            let project_root = std::env::current_dir()?;
            // Check if it's a milestone or task. For now, let's try task first, then milestone.
            if let Err(_) = validation::validate_task(&storage, &project_root, &task_id) {
                 validation::validate_milestone(&storage, &project_root, &task_id)?;
            }
        }
        Commands::Next => {
            let project = storage.load_project()?;
            let mut next_task: Option<(Task, String)> = None; // (Task, Milestone Name)

            // 1. Check Milestones
            for ms in &project.milestones {
                let fragment = storage.load_fragment(&ms.path)?;

                // Prioritize InProgress
                if let Some(task) = fragment.tasks.iter().find(|t| t.status == Status::InProgress) {
                    next_task = Some((task.clone(), ms.name.clone()));
                    break;
                }

                // Then Todo
                if let Some(task) = fragment.tasks.iter().find(|t| t.status == Status::Todo) {
                    next_task = Some((task.clone(), ms.name.clone()));
                    break;
                }

                // Then Backlog (if in a milestone)
                if let Some(task) = fragment.tasks.iter().find(|t| t.status == Status::Backlog) {
                    next_task = Some((task.clone(), ms.name.clone()));
                    break;
                }
            }

            // 2. Check Global Backlog
            if next_task.is_none() {
                let backlog = storage.load_fragment(&project.backlog_path)?;
                if let Some(task) = backlog.tasks.iter().find(|t| t.status != Status::Done && t.status != Status::Canceled) {
                    next_task = Some((task.clone(), "Global Backlog".to_string()));
                }
            }

            if let Some((task, ms_name)) = next_task {
                println!(">>> Next Task: {} - {}", task.id, task.title);
                println!("Milestone: {}", ms_name);
                println!("Status:    {:?}", task.status);
                println!("Priority:  {}", task.priority);

                if !task.designs.is_empty() {
                    println!("\nDesigns:");
                    for design in &task.designs {
                        println!("  - [{:?}] {} (`{:?}`)", design.design_type, design.path, design.status);
                    }
                }

                if !task.metadata.is_empty() {
                    println!("\nMetadata:");
                    for (k, v) in &task.metadata {
                        println!("  {}: {}", k, v);
                    }
                }

                println!("\nSuggested Action:");
                match task.status {
                    Status::Backlog | Status::Todo => {
                        println!("  taskflow-ai execute start {}", task.id);
                    }
                    Status::InProgress => {
                        println!("  taskflow-ai execute complete {}", task.id);
                    }
                    Status::Review => {
                        println!("  taskflow-ai status {} done", task.id);
                    }
                    _ => {}
                }
            } else {
                println!("No pending tasks found. Project complete or backlog empty!");
            }
        }
    }

    Ok(())
}
