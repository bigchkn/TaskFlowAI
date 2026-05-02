use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskType {
    Feature,
    Bug,
    Chore,
    Research,
    Milestone,
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
    pub id: String,           // Human-readable ID (e.g., TF-1, 1.1)
    pub uid: Uuid,            // Immutable internal ID
    pub title: String,
    pub description: String,
    pub task_type: TaskType,
    pub status: Status,
    #[serde(default)]
    pub priority: u8,         // 0-255, higher is more urgent
    
    pub parent_id: Option<Uuid>,
    #[serde(default)]
    pub subtask_uids: Vec<Uuid>,
    
    #[serde(default)]
    pub tags: Vec<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    
    #[serde(default)]
    pub execution: Execution,
    
    #[serde(default)]
    pub metadata: IndexMap<String, String>, // Flexible AI context
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub version: String,
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
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            completed_at: None,
            execution: Execution::default(),
            metadata: IndexMap::new(),
        }
    }
}
