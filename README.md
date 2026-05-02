# TaskFlowAI

Task Management system designed to improve results from AI collaboration.

## Core Philosophy

TaskFlowAI is a specialized task management engine that bridges the gap between high-level project goals and granular execution by AI agents. It improves upon the original `aegis-taskflow` concepts by providing:

- **AI-First Metadata**: Native support for attaching AI context, design references, and execution logs to tasks.
- **Hierarchical Tasking**: Robust support for nested tasks (Milestones → Epics → Tasks → Subtasks).
- **Git-Friendly Storage**: Modular TOML-based storage designed to minimize merge conflicts and remain human-readable.
- **Execution Tracking**: Built-in tracking for which agent performed a task, when, and what the outcome was.

## Project Structure

- `src/model.rs`: Core data structures for Projects, Tasks, and Execution state.
- `src/storage.rs`: Persistence layer (currently supporting local TOML storage).
- `src/main.rs`: CLI interface for human and agent interaction.

## Getting Started

### Initialize a project
```bash
taskflow-ai init "My Awesome Project"
```

### Add a task
```bash
taskflow-ai add "Implement core parser" --task-type feature
```

### List tasks
```bash
taskflow-ai list
```

## Status: Initial Approach

Currently, the project supports basic task creation and listing with a unified storage model. The next phase will focus on modular "fragment" storage and automated Markdown roadmap generation.
