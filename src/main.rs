mod model;
mod roadmap;
mod storage;
mod validation;
mod commands;

use crate::storage::FileStorage;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "taskflow-ai")]
#[command(version = "0.1.0")]
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
        #[arg(short, long)]
        parent: Option<String>,
        #[arg(short = 'T', long)]
        template: Option<String>,
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

    /// Manage the TaskFlowAI agent skill
    Skill {
        #[command(subcommand)]
        command: Option<SkillCommands>,
    },

    /// Move a task to a specific milestone
    Move {
        task_id: String,
        #[arg(short, long)]
        milestone: String,
    },

    /// Delete a task
    Delete { task_id: String },

    /// Edit a task interactively
    Edit { task_id: String },

    /// Archive a completed milestone
    Archive { milestone_id: String },
    /// Sync and regenerate Markdown roadmap files
    Sync,
    /// Show project dashboard
    Dashboard,
    /// Show detailed information about a task
    Show { task_id: String },
    /// Get or set project-level configuration
    Config {
        key: String,
        value: Option<String>,
    },
    /// Manage task templates
    Templates {
        #[command(subcommand)]
        command: TemplateCommands,
    },
}

#[derive(Subcommand)]
enum TemplateCommands {
    /// List all available templates
    List,
    /// Show details of a specific template
    Show { name: String },
}

#[derive(Subcommand)]
enum MilestoneCommands {
    /// Create a new milestone
    Create {
        id: String, // e.g., M1
        name: String,
        #[arg(short, long, default_value_t = 0)]
        priority: u8,
    },
    /// Edit an existing milestone
    Edit {
        id: String,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        priority: Option<u8>,
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
    /// Manage design templates
    Templates {
        #[command(subcommand)]
        command: DesignTemplateCommands,
    },
}

#[derive(Subcommand)]
enum DesignTemplateCommands {
    /// List available design types and their local template status
    List,
    /// Show requirements and template content for a specific design type
    Show {
        /// The design type (hld, lld, rfc)
        design_type: String,
    },
}

#[derive(Subcommand)]
pub enum SkillCommands {
    /// View the skill prompt (default)
    View,
    /// Install the skill for a specific AI provider
    Install {
        /// The AI provider (claude, gemini, codex, dirac, opencode)
        provider: String,
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    let storage_root = std::env::current_dir()?.join(".taskflow");
    let storage = FileStorage::new(storage_root.clone());

    match cli.command {
        Commands::Init { name } => commands::init(&storage, name),
        Commands::Add {
            title,
            task_type,
            milestone,
            parent,
            template,
        } => commands::add(&storage, title, task_type, milestone, parent, template),
        Commands::List { milestone } => commands::list(&storage, milestone),
        Commands::Status {
            task_id,
            new_status,
        } => commands::status(&storage, task_id, new_status),
        Commands::Archive { milestone_id } => commands::archive(&storage, &storage_root, milestone_id),
        Commands::Sync => commands::sync(&storage),
        Commands::Dashboard => commands::dashboard(&storage),
        Commands::Milestone { command } => match command {
            MilestoneCommands::Create { id, name, priority } => commands::milestone_create(&storage, id, name, priority),
            MilestoneCommands::Edit { id, name, priority } => commands::milestone_edit(&storage, id, name, priority),
            MilestoneCommands::List => commands::milestone_list(&storage),
        },
        Commands::Execute { command } => match command {
            ExecuteCommands::Start { task_id, agent } => commands::execute_start(&storage, task_id, agent),
            ExecuteCommands::Complete {
                task_id,
                outcome,
                log,
            } => commands::execute_complete(&storage, task_id, outcome, log),
        },
        Commands::Meta { command } => match command {
            MetaCommands::Set {
                task_id,
                key,
                value,
            } => commands::meta_set(&storage, task_id, key, value),
        },
        Commands::Design { command } => match command {
            DesignCommands::Init {
                design_type,
                title,
                milestone,
                task,
            } => commands::design_init(&storage, design_type, title, milestone, task),
            DesignCommands::Status {
                path,
                status,
                milestone,
                task,
            } => commands::design_set_status(&storage, path, status, milestone, task),
            DesignCommands::Templates { command } => match command {
                DesignTemplateCommands::List => commands::design::templates_list(),
                DesignTemplateCommands::Show { design_type } => commands::design::templates_show(&design_type),
            },
        },

        Commands::Validate { task_id } => commands::validate(&storage, task_id),
        Commands::Next => commands::next(&storage),
        Commands::Skill { command } => commands::skill(command),
        Commands::Move {
            task_id,
            milestone,
        } => commands::move_task(&storage, task_id, milestone),
        Commands::Delete { task_id } => commands::delete(&storage, task_id),
        Commands::Edit { task_id } => commands::edit(&storage, task_id),
        Commands::Show { task_id } => commands::show(&storage, task_id),
        Commands::Config { key, value } => commands::config(&storage, key, value),
        Commands::Templates { command } => match command {
            TemplateCommands::List => commands::templates::list(),
            TemplateCommands::Show { name } => commands::templates::show(&name),
        },
    }
}
