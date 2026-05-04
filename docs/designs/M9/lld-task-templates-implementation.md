# Low-Level Design: Task Templates Implementation

## 1. Objective
Implement Task Templates to standardize workflows by pre-defining required task metadata, expected subtasks, and required documentation. This ensures that different types of work (e.g., Features, Bugfixes, Research) follow consistent processes and contain necessary context before execution.

## 2. Architecture
The Task Template system will consist of:
- **Template Storage**: TOML files located in `.taskflow/templates/tasks/` (e.g., `feature.toml`, `bugfix.toml`).
- **Template Application**: Logic in the `add` command to read templates and scaffold tasks.
- **Enforcement Logic**: Validation rules in the `validate` command to ensure tasks adhere to their template's constraints.

### Data Model (Proposed TOML)
```toml
name = "Feature"
description = "Standard template for new features"

[required_metadata]
priority_reason = "string"
impact_analysis = "string"

[[default_subtasks]]
title = "Write LLD"
task_type = "research"

[[default_subtasks]]
title = "Implementation"
task_type = "feature"

[[required_designs]]
design_type = "lld"
```

## 3. Implementation Details
- **Template Parsing**: Create a `TaskTemplate` struct in `model.rs` that matches the TOML structure.
- **`add` Command Update**: 
    - Add a `--template` flag to `taskflow-ai add`.
    - If provided, load the TOML from `.taskflow/templates/tasks/<name>.toml`.
    - Automatically create subtasks using the existing hierarchical support.
    - Populate `task.metadata` with keys from `required_metadata`.
- **`validate` Command Update**:
    - Extend `validate_task` to check if a task specifies a template in its metadata.
    - Verify all `required_metadata` keys are present and non-empty.
    - Verify all `required_designs` types are linked to the task.
- **Config**: Add a `default_template` project configuration to automatically apply a template if `--template` is omitted.

## 4. Verification Plan
- **Unit Tests**: Test template parsing and validation logic.
- **Integration Tests**:
    - Run `taskflow-ai add "New Feature" --template feature`.
    - Verify subtasks are created and metadata keys are initialized.
    - Run `taskflow-ai validate TF-X` and verify it fails if metadata is missing.
    - Fill metadata and verify `validate` passes.
