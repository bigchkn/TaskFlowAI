use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Backlog,
    Todo,
    InProgress,
    Review,
    Done,
    Blocked,
    Canceled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum TaskType {
    Feature,
    Bug,
    Chore,
    Research,
    Task, // General task
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum DesignType {
    Hld,
    Lld,
    Rfc,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum DesignStatus {
    Draft,
    Review,
    Approved,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Design {
    pub design_type: DesignType,
    pub path: String,
    pub status: DesignStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Execution {
    pub agent_id: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub logs: Vec<String>,
    pub outcome: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,           // Human-readable ID (e.g., TF-1)
    pub uid: Uuid,            // Immutable internal ID
    pub title: String,
    pub description: String,
    pub task_type: TaskType,
    pub status: Status,
    #[serde(default)]
    pub priority: u8,         
    
    pub parent_id: Option<Uuid>,
    #[serde(default)]
    pub subtask_uids: Vec<Uuid>,
    
    pub milestone_id: Option<String>, // ID of the milestone fragment (e.g., "M1")
    
    #[serde(default)]
    pub designs: Vec<Design>,
    
    #[serde(default)]
    pub tags: Vec<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    
    #[serde(default)]
    pub execution: Execution,
    
    #[serde(default)]
    pub metadata: IndexMap<String, String>, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneMetadata {
    pub id: String,          // e.g., "M1"
    pub name: String,
    pub description: String,
    pub target_date: Option<DateTime<Utc>>,
    pub status: Status,
    pub path: String,        // e.g., "roadmap/M1.toml"
    #[serde(default)]
    pub designs: Vec<Design>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub version: String,
    pub milestones: Vec<MilestoneMetadata>,
    #[serde(default)]
    pub archived_milestones: Vec<MilestoneMetadata>,
    pub backlog_path: String, // e.g., "roadmap/backlog.toml"
    
    #[serde(default)]
    pub config: IndexMap<String, String>,
}

/// A fragment containing a collection of tasks (Milestone or Backlog)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskFragment {
    pub tasks: Vec<Task>,
}

impl Task {
    pub fn new(id: String, title: String, task_type: TaskType) -> Self {
        let now = Utc::now();
        Self {
            id,
            uid: Uuid::new_v4(),
            title,
            description: String::new(),
            task_type,
            status: Status::Backlog,
            priority: 0,
            parent_id: None,
            subtask_uids: Vec::new(),
            milestone_id: None,
            designs: Vec::new(),
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            completed_at: None,
            execution: Execution::default(),
            metadata: IndexMap::new(),
        }
    }
}
