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
```
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

## Project Structure

- `src/model.rs`: Core data structures using `IndexMap` for deterministic serialization.
- `src/storage.rs`: Persistence layer for fragmented TOML state.
- `src/roadmap.rs`: Markdown generation logic for active and archived views.
- `src/validation.rs`: Template enforcement and design document checking.
- `src/main.rs`: CLI entry point.
