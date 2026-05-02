# Low-Level Design: Edit Command Implementation

## 1. Objective
Implement an `edit` command that allows interactive editing of a task's metadata and description by opening the relevant TOML fragment (or a temporary representation of the task) in the user's preferred text editor.

## 2. Architecture
The `edit` command will:
1.  **Locate Task**: Find the fragment containing the task.
2.  **Extract Task**: Isolate the specific task's TOML representation.
3.  **Spawn Editor**:
    -   Create a temporary file with the task's TOML.
    -   Read the `EDITOR` or `VISUAL` environment variable (default to `vi` or `nano`).
    -   Open the temporary file in the editor and wait for the process to exit.
4.  **Parse & Update**:
    -   Read the modified temporary file.
    -   Validate the TOML structure and ensure the Task ID hasn't changed (or handle it).
    -   Update the task in the original fragment.
5.  **Sync**: Regenerate the roadmap.

## 3. Implementation Details
-   **CLI Signature**: `taskflow-ai edit <TASK_ID>`
-   **Dependencies**: `tempfile` crate (check if already in `Cargo.toml`).
-   **Editor Spawning**: Use `std::process::Command`.

## 4. Verification Plan
1.  Run `taskflow-ai edit TF-X`.
2.  Modify the description or a metadata field in the editor.
3.  Save and exit.
4.  Verify the changes are reflected in the TOML file and `taskflow-ai dashboard`.
