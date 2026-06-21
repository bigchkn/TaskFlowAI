# Low-Level Design: Create built-in task type task which does not strictly require LLD

## 1. Objective
Currently, `taskflow-ai` has built-in templates `feature` (which requires an LLD) and `research` (which requires an RFC). For smaller, straightforward tasks that do not need design documents but still need metadata validation and default subtasks, there is no generic template. 

The objective is to introduce a built-in template named `task` which:
1. Does not require any design documents (`required_designs` is empty).
2. Requires `priority_reason` in `required_metadata`.
3. Automatically scaffolds a single default subtask: `title = "Implementation"`, `task_type = "feature"`.

## 2. Architecture
The new template will be defined statically in the `DEFAULT_TEMPLATES` array within `src/commands/templates.rs`. This ensures it behaves exactly like `feature` and `research`, supporting local overrides, automatic materialization during `task templates init`, and listing via `task templates list`.

## 3. Implementation Details
Update `src/commands/templates.rs`:
- Append a new `DefaultTemplate` entry to `DEFAULT_TEMPLATES`:
```rust
    DefaultTemplate {
        name: "task",
        filename: "task.toml",
        content: r#"name = "Task"
description = "Standard template for general tasks without design requirements"

[required_metadata]
priority_reason = "string"

[[default_subtasks]]
title = "Implementation"
task_type = "feature"
"#,
    },
```

No other code changes are needed because the validation and parsing system automatically resolves any built-in template by name via `get_template`.

## 4. Verification Plan
1. **Validate design document**:
   Run `cargo run -- task validate TF-61` to ensure this LLD complies with design headers.
2. **List templates**:
   Run `cargo run -- task templates list` and verify `task` is present in the list.
3. **Show template**:
   Run `cargo run -- task templates show task` and verify it displays the description and single `Implementation` subtask.
4. **Create a task with the template**:
   Run `cargo run -- task add "Verify Task Template" -T task` and verify that the task and its child subtask `Implementation` are created.
5. **Verify metadata validation**:
   Run `cargo run -- task validate TF-X` (for the created task). It should fail because `priority_reason` is empty.
   Run `cargo run -- task meta set TF-X priority_reason "Simple change"`.
   Run `cargo run -- task validate TF-X` again. It should succeed without demanding any LLD or RFC documents.

