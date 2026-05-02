# Low-Level Design: Hierarchical Subtask Support

## 1. Objective
Enable the creation of hierarchical task structures using a `--parent` flag during task creation. This allows complex tasks to be broken down into smaller, manageable subtasks while maintaining a clear relationship in the metadata and visualizations.

## 2. Architecture
1.  **Relationship Storage**:
    -   Subtasks store their parent's ID in `parent_id`.
    -   Parents store a list of their children's UUIDs in `subtask_uids` for fast lookup and integrity.
2.  **Creation Logic**:
    -   When `taskflow-ai add --parent <PARENT_ID>` is called:
        -   Locate the parent task across all fragments.
        -   Create the new subtask with `parent_id` set to `<PARENT_ID>`.
        -   Update the parent task's `subtask_uids` to include the new subtask's UID.
3.  **Cross-Fragment Integrity**:
    -   The parent and child can reside in different fragments (e.g., a Milestone task having a Backlog subtask). The `Storage` layer must handle loading and saving both fragments atomically where possible.

## 3. Implementation Details
-   **CLI Update**: Add `parent: Option<String>` to the `Add` subcommand in `src/main.rs`.
-   **Linkage Logic**:
    -   Use `storage.find_task_path` to find the parent.
    -   Update parent's `subtask_uids`.
    -   Ensure the subtask inherits the milestone of the parent by default (optional, but recommended).

## 4. Verification Plan
1.  Create a parent task: `taskflow-ai add "Parent Task" --milestone M3`. (TF-15)
2.  Create a subtask: `taskflow-ai add "Subtask A" --parent TF-15`. (TF-16)
3.  Verify `TF-16` has `parent_id = "TF-15"`.
4.  Verify `TF-15` has `subtask_uids` containing `TF-16`'s UID.
5.  Check `ROADMAP_ACTIVE.md` to see if hierarchy is visually indicated (may require roadmap generator updates).
