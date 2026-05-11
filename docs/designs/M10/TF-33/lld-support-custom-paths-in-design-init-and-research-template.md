# Low-Level Design: Support custom paths in design init and research template

## 1. Objective
Support an input queue for research and design questions by:
1. Enhancing `taskflow-ai design init` to support an optional `--path` flag for custom output directories (e.g., `docs/research/`).
2. Adding a `research.toml` task template to standardize the inquiry process.

## 2. Architecture
- **CLI**: Add an optional `--path` (or `-p`) argument to the `DesignCommands::Init` subcommand in `src/main.rs`.
- **Logic**: Update `src/commands/design.rs` to respect the provided path if it exists, falling back to the default `docs/designs/` logic.
- **Templates**: Create `.taskflow/templates/tasks/research.toml`.

## 3. Implementation Details

### 3.1 CLI Update (`src/main.rs`)
```rust
#[derive(Subcommand)]
enum DesignCommands {
    /// Initialize a new design document
    Init {
        design_type: String,
        title: String,
        #[arg(short, long)]
        milestone: String,
        #[arg(short, long)]
        task: Option<String>,
        #[arg(short, long)]
        path: Option<String>, // New optional path
    },
    // ...
}
```

### 3.2 Logic Update (`src/commands/design.rs`)
Update the `init` function signature and path construction:
```rust
pub fn init<S: Storage>(
    storage: &S,
    design_type: String,
    title: String,
    milestone: String,
    task: Option<String>,
    path: Option<String>, // New parameter
) -> Result<()> {
    // ...
    let mut relative_path = if let Some(custom_path) = path {
        PathBuf::from(custom_path)
    } else {
        let mut p = PathBuf::from("docs/designs");
        p.push(&milestone);
        if let Some(ref t_id) = task {
            p.push(t_id);
        }
        p
    };
    // ...
}
```

### 3.3 New Template (`.taskflow/templates/tasks/research.toml`)
```toml
name = "Research"
description = "Template for research, spikes, and architecture inquiries."

[required_metadata]
research_goal = "The primary question or objective of this research."
research_path = "Suggested: docs/research/"

[[required_designs]]
design_type = "rfc"

[[default_subtasks]]
title = "Initial Research & Discovery"
task_type = "research"

[[default_subtasks]]
title = "Document Findings"
task_type = "research"
```

## 4. Verification Plan
1. **Custom Path**: Run `cargo run -- design init rfc "Test Research" --milestone M10 --path docs/research`. Verify the file is created at `docs/research/rfc-test-research.md`.
2. **Template Usage**: Run `cargo run -- add "How to scale X?" -T research`. Verify it creates the task with the required metadata fields.
3. **Validation**: Run `cargo run -- validate TF-X` for the new research task. It should fail if the RFC is missing or if `research_goal` is empty.
