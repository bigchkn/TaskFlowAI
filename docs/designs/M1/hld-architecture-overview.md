# TaskFlowAI - High Level Design (HLD)

## 1. Introduction
TaskFlowAI is a specialized task management engine that bridges the gap between high-level project goals and granular execution by AI agents. By providing structured, modular, and version-controllable task states, it ensures autonomous agents remain aligned with project architecture and requirements.

### 1.1 Core Tenets
*   **Decoupling Metadata from Documentation:** A fundamental principle of TaskFlowAI is that machine-readable state (metadata, execution tracking, IDs) must be strictly decoupled from human-readable content (design docs, long-form descriptions). Metadata lives in TOML fragments; narrative documentation lives in Markdown. The engine acts as the strict bridge between them.
*   **AI-First but Human-Readable:** The system must be perfectly understandable by both autonomous agents and human developers without needing proprietary web interfaces.
*   **Scalability over Time:** The system must remain performant and legible even as the number of completed milestones and tasks "explodes" over a project's lifecycle.

## 2. Current Architecture
The initial implementation establishes the core primitives for AI-friendly task management:

### 2.1 Core Models
*   **Task**: The fundamental unit of work. Includes:
    *   Human-readable `id` (e.g., TF-1) and immutable internal `uid` (UUID).
    *   `parent_id` and `subtask_uids` for hierarchical nesting (Milestone -> Epic -> Task -> Subtask).
    *   `metadata`: A flexible key-value store for AI context (e.g., prompt hints, design document links).
    *   `Execution`: Tracks the agent assigned, timestamps, execution logs, and outcomes.
*   **Project**: The root container, maintaining global state.

### 2.2 Storage Layer & CLI Interface
*   Provides basic commands (`init`, `add`, `list`, `status`) backed by a unified `project.toml` file.

---

## 3. Planned Enhancements

To enforce consistency and manage complexity over time, TaskFlowAI will introduce template-driven workflows and scalable storage.

### 3.1 Design Templates (Document Enforcement)
Design Templates enforce the structure and content of essential project documentation (e.g., HLD, LLD, API schemas).

*   **Mechanism**: Projects will store Markdown templates (e.g., `lld-template.md`) in a `.taskflow/templates/designs/` directory.
*   **Enforcement**: 
    *   When a task transitions to an execution state (e.g., `InProgress`), the engine will parse the linked design document (e.g., the generated LLD).
    *   It will structurally validate that required sections (defined in the template) are present and sufficiently populated.
*   **AI Integration**: These templates serve a dual purpose as strict system prompts for agents tasked with generating the designs.

### 3.2 Task Templates (Workflow Enforcement)
Task Templates standardize workflows by pre-defining required task metadata, expected subtasks, and required documentation.

*   **Mechanism**: Defined as TOML files in `.taskflow/templates/tasks/` (e.g., `feature.toml`, `bugfix.toml`).
*   **Configuration & Enforcement**:
    *   `required_designs`: Enforces that specific task types must link to specific design documents (e.g., a "Feature" task *must* have an associated LLD).
    *   `default_subtasks`: Automatically scaffolds standardized subtasks (e.g., a "Feature" automatically spawns `[Write LLD, Implement, Test, Review]`).
    *   `required_metadata`: Enforces that certain context fields (e.g., `api_endpoint`) are populated before the task can be marked `ReadyForDev`.

### 3.3 Storage Evolution (Fragmented & Scalable Storage)
To support concurrency, avoid monolithic file conflicts, and maintain readability as the project scales:

*   **Fragmented TOMLs**: Transition from a single `project.toml` to a `roadmap/` directory.
    *   **Index**: `roadmap/index.toml` tracks active milestones/epics.
    *   **Fragments**: Tasks are grouped into individual milestone files (e.g., `roadmap/M1.toml`) or a `roadmap/backlog.toml`.
    *   **Locking**: Implement file-level locking during modifications to support concurrent agent activity.
*   **Archiving Strategy**: To prevent the "explosion" of files from degrading readability over time, the engine will support "Tombstoning" or "Archiving" completed milestones.
    *   Completed milestones are moved from the active `roadmap/` index to an `archive/` directory.
    *   This keeps the active state lean for both human developers and the AI context window.
*   **Decoupled Documentation Syncing**: The engine will generate multiple scoped views instead of one giant file. For example, generating a lean `ROADMAP_ACTIVE.md` for current context, while historical data remains in the TOML archive or a separate `ROADMAP_ARCHIVE.md`.

## 5. CLI Command Reference

TaskFlowAI is primarily interacted with via a CLI. The commands are designed to be composable and easy for both humans and AI agents to use.

### 5.1 Initialization & Configuration
*   `taskflow-ai init [PROJECT_NAME]`: Initializes a new `.taskflow/` directory structure, including default templates and the initial `roadmap/index.toml`.
*   `taskflow-ai config <KEY> [VALUE]`: Gets or sets project-level configuration (e.g., default task template, archive retention policy).

### 5.2 Task & Milestone Management
*   `taskflow-ai add <TITLE> [--type <TYPE>] [--milestone <M_ID>] [--template <TPL_NAME>]`: Creates a new task. If `--milestone` is omitted, it goes to the backlog. If `--template` is provided, it scaffolds subtasks and metadata requirements.
*   `taskflow-ai edit <TASK_ID>`: Opens the task's TOML representation in the default `$EDITOR` for manual modification.
*   `taskflow-ai milestone create <NAME> [--target-date <DATE>]`: Creates a new milestone fragment (e.g., `M2.toml`) and updates the index.
*   `taskflow-ai move <TASK_ID> <MILESTONE_ID>`: Moves a task from the backlog to a milestone, or between milestones.

### 5.3 State & Execution Tracking
*   `taskflow-ai status <TASK_ID> <NEW_STATUS>`: Updates the status of a task (e.g., `Todo`, `InProgress`, `Done`).
*   `taskflow-ai execute start <TASK_ID> [--agent <AGENT_ID>]`: Marks a task as `InProgress` and logs the execution start time and agent ID.
*   `taskflow-ai execute complete <TASK_ID> [--outcome <SUCCESS|FAIL>] [--log <MSG>]`: Marks a task as `Done` (or `Blocked`/`Failed`), logging the end time, outcome, and optional log message.
*   `taskflow-ai meta set <TASK_ID> <KEY> <VALUE>`: Sets a specific metadata key-value pair (e.g., `taskflow-ai meta set TF-1 lld_path docs/lld-auth.md`).

### 5.4 Validation & Syncing
*   `taskflow-ai validate <TASK_ID>`: Checks a task against its Task Template constraints (e.g., ensures `required_metadata` is set, and parses linked Design Documents against their Design Templates).
*   `taskflow-ai sync`: Regenerates the `ROADMAP_ACTIVE.md` and `ROADMAP_ARCHIVE.md` files based on the current TOML state. This is typically run automatically via Git hooks or after state changes, but can be run manually.

### 5.5 Querying & Archiving
*   `taskflow-ai list [--milestone <M_ID>] [--status <STATUS>] [--agent <AGENT_ID>]`: Lists tasks matching the given criteria. Defaults to showing all active tasks.
*   `taskflow-ai show <TASK_ID>`: Displays detailed information about a specific task, including its execution history and metadata.
*   `taskflow-ai archive <MILESTONE_ID>`: Moves a completed milestone from the `roadmap/` directory to the `archive/` directory and updates the index.
