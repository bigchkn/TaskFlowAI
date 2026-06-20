# TaskFlowAI

Task Management system designed for seamless AI-Human collaboration.

## Core Philosophy

TaskFlowAI is a specialized task management engine that bridges the gap between high-level project goals and granular execution by AI agents. It prioritizes machine-readability for agents while maintaining perfect legibility and control for humans.

- **Metadata Decoupling**: Strictly separates machine-readable state (TOML) from human-readable narrative (Markdown).
- **Scalable State**: Prevents "context explosion" through an automated archiving strategy for completed milestones.
- **Git-Native & Fragmented**: Uses modular TOML fragments to eliminate merge conflicts and provide clear version history.
- **Verification-Driven**: Built-in validation ensures that tasks and design documents adhere to project templates.

## Current Features

- **Fragmented Storage**: Tasks are stored in `.taskflow/roadmap/` as individual milestone fragments (`M1.toml`, `backlog.toml`).
- **Milestone Archiving**: Move completed work to `.taskflow/archive/` to keep the active context window lean.
- **Automated Roadmaps**: Synchronizes state to `ROADMAP_ACTIVE.md` and `ROADMAP_ARCHIVE.md` automatically on every change.
- **Execution Tracking**: Log start/end times, agent IDs, and outcomes for every task.
- **Design Validation**: Validate linked design documents (LLDs) against predefined Markdown templates.

## CLI Usage

### Initialization
```bash
taskflow-ai init "My Project"
```

### Milestone Management
```bash
taskflow-ai milestone create M1 "Foundational Phase"
taskflow-ai milestone list
```

### Task Lifecycle
```bash
# Add to backlog
taskflow-ai task add "Implement authentication" 

# Add to specific milestone
taskflow-ai task add "Setup database" --milestone M1

# Update status
taskflow-ai task status TF-1 in-progress
```

### Execution & Metadata
```bash
# Track agent activity
taskflow-ai task execute start TF-1 --agent "Gemini"
taskflow-ai task execute complete TF-1 --outcome "Success" --log "Database schema applied."

# Link design docs
taskflow-ai task meta set TF-1 "lld_path" "docs/lld-auth.md"

# Validate requirements
taskflow-ai task validate TF-1
```

### Archiving & Sync
```bash
# Archive completed milestone
taskflow-ai milestone archive M1

# Manual roadmap sync (usually automatic)
taskflow-ai sync

# Project dashboard
taskflow-ai dashboard
```

### Shell Completions
```bash
# Print a completion script for a supported shell
taskflow-ai completions bash
taskflow-ai completions zsh
taskflow-ai completions fish
```

## Configuration

Project configuration is stored in `.taskflow/roadmap/index.toml` and managed with `taskflow-ai config`.

```bash
# Read a value
taskflow-ai config document_dir

# Set a value
taskflow-ai config document_dir docs/designs
```

Key configurations:

- `default_template`: Template applied to new root tasks when `--template` is omitted.
- `force_templates`: Set to `true` to require templates for new root tasks.
- `document_dir`: Directory used by `taskflow-ai design init` when `--path` is omitted. Defaults to `docs/designs`.
- `roadmap_active_path`: Output path for the active roadmap. Defaults to `ROADMAP_ACTIVE.md`.
- `roadmap_archive_path`: Output path for the archived roadmap. Defaults to `ROADMAP_ARCHIVE.md`.

## Templates

TaskFlowAI supports two types of templates to structure and validate your workflow: **Task Templates** and **Design Templates**.

### Task Templates
Task templates define required metadata fields, automatically scaffolded subtasks, and design requirements for a task type.

* **Initialization**: Run `taskflow-ai task templates init` to write the default templates (`feature.toml`, `research.toml`) to `.taskflow/templates/tasks/`.
* **Customization**: Create or modify TOML files directly under `.taskflow/templates/tasks/`.
* **CLI Usage**:
  ```bash
  # List all available task templates
  taskflow-ai task templates list

  # Show required metadata and subtasks for a template
  taskflow-ai task templates show feature

  # Add a task using a template
  taskflow-ai task add "New Endpoint" -T feature
  ```

### Design Templates
Design templates provide standard Markdown structure for design files (HLDs, LLDs, RFCs) and enforce specific headers during validation.

* **Initialization**: Run `taskflow-ai design templates init` to write defaults (`hld.md`, `lld.md`, `rfc.md`) to `.taskflow/templates/designs/`.
* **Customization**: Create or edit Markdown files directly under `.taskflow/templates/designs/`.
* **CLI Usage**:
  ```bash
  # List design templates and check if local files exist
  taskflow-ai design templates list

  # Show required headers and template layout
  taskflow-ai design templates show lld

  # Scaffold a design doc linked to a milestone or task
  taskflow-ai design init lld "Database Schema" --milestone M1 --task TF-1
  ```

## Project Structure

- `src/model.rs`: Core data structures using `IndexMap` for deterministic serialization.
- `src/storage.rs`: Persistence layer for fragmented TOML state.
- `src/roadmap.rs`: Markdown generation logic for active and archived views.
- `src/validation.rs`: Template enforcement and design document checking.
- `src/main.rs`: CLI entry point.
