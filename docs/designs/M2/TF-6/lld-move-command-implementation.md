# Low-Level Design: Move Command Implementation

## 1. Objective
Implement the `move` command to allow transferring tasks between different storage fragments. The primary use case is moving tasks from the global `backlog.toml` into a specific milestone fragment (e.g., `M2.toml`), or between milestones.

## 2. Architecture
The `move` command will:
1.  **Locate Source**: Find the task in the project (either in a milestone or the global backlog).
2.  **Verify Destination**: Ensure the target milestone exists.
3.  **Atomic Transfer**:
    -   Remove the task from the source fragment.
    -   Add the task to the target fragment.
    -   Update the task's status to `Todo` if it was in `Backlog` and is being moved into a milestone (optional, but follows common workflow).
4.  **Save Changes**: Persist both modified fragments.

## 3. Implementation Details
-   **CLI Signature**: `taskflow-ai move <TASK_ID> --milestone <M_ID>`
-   **Storage Logic**:
    -   Extend `Storage` trait/impl to support task removal.
    -   Use `Project::find_task_path` to locate the source file.
    -   Update `ROADMAP_ACTIVE.md` via `sync` after the move.

## 4. Verification Plan
1.  Add a task to the backlog: `taskflow-ai add "Test Task"`.
2.  Move it to a milestone: `taskflow-ai move TF-X --milestone M2`.
3.  Verify the task is no longer in `backlog.toml` and exists in `M2.toml`.
4.  Run `taskflow-ai dashboard` to confirm visual update.
