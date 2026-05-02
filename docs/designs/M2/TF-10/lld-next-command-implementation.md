# Low-Level Design: Next Command Implementation

## 1. Objective
The `next` command aims to provide an immediate, actionable entry point for developers and AI agents. It identifies the most relevant task to work on next based on milestone order, task status, and priority, presenting all necessary context (designs, metadata) in a single view.

## 2. Architecture
The command will:
1. Load the project and all active milestones.
2. Iterate through milestones in their indexed order.
3. For the first incomplete milestone:
   - Search for tasks with status `InProgress`. If found, suggest continuing this task.
   - If no `InProgress` tasks, find the first task with status `Todo`.
   - If no `Todo` tasks, find the first task with status `Backlog`.
4. If no tasks are found in milestones, check the global backlog.

## 3. Implementation Details
- **Command**: `taskflow-ai next`
- **Output Components**:
    - **Header**: ">>> Next Task: [ID] - [TITLE]"
    - **Context**: Milestone Name, Task Status, Priority.
    - **Designs**: List all linked HLD/LLD paths with their current status.
    - **Metadata**: Display key metadata fields if present.
    - **Action**: A suggested CLI command to transition to the next state (e.g., if status is `Todo`, suggest `execute start`).

## 4. Verification Plan
- **Standard Case**: Verify it picks the first `Todo` task in `M1`.
- **In-Progress Case**: Verify it prioritizes an `InProgress` task over a `Todo` task.
- **Empty Milestone Case**: Verify it skips empty or fully completed milestones and checks the next one.
- **Backlog Fallback**: Verify it suggests a task from the global backlog if all milestones are completed.
