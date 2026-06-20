# Low-Level Design: Task templates do not show included templates

## 1. Objective
Currently, `taskflow-ai task templates list` only scans the `.taskflow/templates/tasks/` directory. If the directory is missing or empty, it claims "No templates found". However, the system has built-in (pre-built) default templates: `feature` and `research`. 

The objective of this design is to update the task template management commands (`list`, `show`) and the task creation flow (`add`, `validate`) to:
1. Recognize, list, show, and resolve built-in templates (`feature` and `research`) even if the `.taskflow/templates/tasks` directory has not been initialized or is empty.
2. Clearly indicate the source of each template in the list view (e.g., `Built-in`, `Local`, or `Local Override`).
3. Ensure task creation (`add`) and validation (`validate`) flows correctly fall back to built-in templates if no local overrides are present.

## 2. Architecture
A centralized helper module and function will be introduced or refined to load and parse templates. The template resolution strategy will be:
1. Check the local `.taskflow/templates/tasks/<name>.toml` file first.
2. If the local file exists, deserialize it. If the local file overrides a built-in template, mark its source as `Local Override` (or `Local` if it is a custom template name).
3. If no local file exists but the template name matches a built-in template (`feature` or `research`), parse the built-in default TOML structure and return it.
4. If neither is found, return `None` (not found).

This logic will be integrated into:
- [src/commands/templates.rs](file:///Users/Mattew/ws/taskflow/src/commands/templates.rs): `list` (lists all built-in and local templates), `show` (renders template details from the resolved template), and `get_template` helper.
- [src/commands/add.rs](file:///Users/Mattew/ws/taskflow/src/commands/add.rs): Resolves the task template during task addition, allowing fallbacks.
- [src/validation.rs](file:///Users/Mattew/ws/taskflow/src/validation.rs): Resolves the task template during task verification.

## 3. Implementation Details

### A. Shared Helper in `templates.rs`
Define `DEFAULT_TEMPLATES` containing the TOML contents of the pre-built templates.
Implement `get_template(project_root: &Path, name: &str) -> Option<TaskTemplate>` which performs the cascading lookup.

### B. List Command `list()` in `templates.rs`
Collect all unique template names from both default templates and the local `.taskflow/templates/tasks` directory (if it exists).
For each template, compute the source:
- If present in defaults but not local: `Built-in`
- If present in both defaults and local: `Local Override`
- If present only in local: `Local`
Display a table containing `TEMPLATE | SOURCE | DESCRIPTION`.

### C. Show Command `show()` in `templates.rs`
Retrieve the template via `get_template`. Compute the detailed source info (path or `Built-in`) and print the template details.

### D. Update `add.rs`
Use `crate::commands::templates::get_template` (or inline fallback logic) to resolve the template during `task add`.

### E. Update `validation.rs`
Use `crate::commands::templates::get_template` to resolve the template during `task validate`.

## 4. Verification Plan

### Automated/Manual Validation
1. **Validate Design LLD**:
   Run `cargo run -- task validate TF-54` to ensure the LLD is recognized.
2. **Verify list command (before initialization)**:
   Rename/remove `.taskflow/templates/tasks` if present, run `cargo run -- task templates list` and verify it lists `feature` and `research` as `Built-in`.
3. **Verify show command (before initialization)**:
   Run `cargo run -- task templates show feature` and verify it shows the details with `Source: Built-in`.
4. **Verify task creation with built-in template (before initialization)**:
   Create a new task using `cargo run -- task add "Test Built-in Feature" -T feature`. Verify the task is created and subtasks from the template are automatically generated.
5. **Verify templates init**:
   Run `cargo run -- task templates init` to materialize the templates locally.
6. **Verify list command (after initialization)**:
   Run `cargo run -- task templates list` and verify `feature` and `research` now show as `Local Override`.
7. **Verify overriding behaviour**:
   Modify the local `.taskflow/templates/tasks/feature.toml` file to add a new subtask. Run `cargo run -- task add "Test Override Feature" -T feature` and verify the new subtask is present.

