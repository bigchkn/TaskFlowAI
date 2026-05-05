use crate::SkillCommands;
use anyhow::{Context, Result, bail};
use std::fs;

pub const TASKFLOW_SKILL_PROMPT: &str = r#"---
name: taskflow
description: TaskFlowAI agent skill for hierarchical task management and design synchronization.
---
# TaskFlowAI Agent Skill

You are an AI agent operating within a TaskFlowAI managed project. Your goal is to manage tasks and designs while maintaining the strict decoupling of machine-readable metadata and human-readable documentation.

## Core Principles
1. **Metadata vs. Documentation**: Task state (status, priority, execution logs) lives in TOML fragments in `.taskflow/roadmap/`. Narrative documentation (HLDs, LLDs) lives in `docs/designs/`.
2. **Git-Native**: All state changes must be committed to Git.
3. **Automated Roadmaps**: `ROADMAP_ACTIVE.md` is automatically generated from the TOML fragments. Never edit it manually.

## Standard Workflow
1. **Discover**: 
   - Run `taskflow-ai next` to find the highest priority task and its associated designs.
   - Run `taskflow-ai templates list` to see available task types if creating a new task.
2. **Research & Design**:
   - Read the linked HLD/LLDs.
   - Use `taskflow-ai design templates show <type>` to view required headers for a design type.
   - If a new design is needed, run `taskflow-ai design init <hld|lld> <Title> --milestone <M_ID> [--task <TF_ID>]`.
   - Populate the scaffolded Markdown file.
3. **Execute**:
   - Start: `taskflow-ai execute start <TF_ID> --agent <Agent_Name>`.
   - Implement the change.
   - Validate: `taskflow-ai validate <TF_ID>`.
   - Complete: `taskflow-ai execute complete <TF_ID> --outcome success --log "Summary of work"`.
4. **Sync**: The roadmap usually syncs automatically, but you can run `taskflow-ai sync` to be sure.

## Directory Structure
- `.taskflow/roadmap/index.toml`: Project metadata and milestone index.
- `.taskflow/roadmap/M*.toml`: Milestone fragments containing tasks.
- `.taskflow/templates/designs/`: Markdown templates for HLD/LLD.
- `.taskflow/templates/tasks/`: TOML templates for different task types.
- `docs/designs/`: Human-readable design documents.
- `ROADMAP_ACTIVE.md`: The generated project view.
"#;

pub fn run(command: Option<SkillCommands>) -> Result<()> {
    let cmd = command.unwrap_or(SkillCommands::View);

    match cmd {
        SkillCommands::View => {
            println!("{}", TASKFLOW_SKILL_PROMPT);
        }
        SkillCommands::Install { provider } => {
            let home = std::env::var("HOME").context("Could not find HOME environment variable")?;

            let skill_path = match provider.to_lowercase().as_str() {
                "claude" => format!("{}/.claude/skills/taskflow/SKILL.md", home),
                "gemini" => format!("{}/.gemini/skills/taskflow/SKILL.md", home),
                "codex" => format!("{}/.codex/skills/taskflow/SKILL.md", home),
                "dirac" => format!("{}/.dirac/skills/taskflow/SKILL.md", home),
                "opencode" => format!("{}/.config/opencode/skill/taskflow/SKILL.md", home),
                _ => bail!(
                    "Unsupported provider: {}. Supported providers are: claude, gemini, codex, dirac, opencode",
                    provider
                ),
            };

            let path = std::path::Path::new(&skill_path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {:?}", parent))?;
            }

            fs::write(path, TASKFLOW_SKILL_PROMPT)
                .with_context(|| format!("Failed to write skill to: {:?}", path))?;
            println!("Successfully installed TaskFlowAI skill to {}", skill_path);
        }
    }
    Ok(())
}
