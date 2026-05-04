# Low-Level Design: Templates Command Implementation

# Low-Level Design: Templates Command Implementation

## 1. Objective
Provide a CLI command to list and show details of available task templates. This improves discoverability for both human users and AI agents, especially when `force_templates` is enabled.

## 2. Architecture
The `templates` command will scan the `.taskflow/templates/tasks/` directory and parse the TOML files to display metadata.

### Subcommands
- `taskflow-ai templates list`: Lists names and descriptions of all templates.
- `taskflow-ai templates show <NAME>`: Shows detailed information about a specific template (required metadata, default subtasks).

## 3. Implementation Details
- **Command Handler**: Create `src/commands/templates.rs`.
- **CLI Definition**: Update `src/main.rs` to include the `Templates` subcommand.
- **Logic**:
    - Use `std::fs::read_dir` on `.taskflow/templates/tasks/`.
    - Filter for `.toml` files.
    - Parse using `crate::model::TaskTemplate`.
    - Format output for the user.

## 4. Verification Plan
1. Run `taskflow-ai templates list`.
2. Verify `feature` template is listed.
3. Run `taskflow-ai templates show feature`.
4. Verify required metadata and subtasks are displayed.
