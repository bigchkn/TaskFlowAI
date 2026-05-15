use crate::SkillCommands;
use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

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
4. **Command-Driven State**: Do not modify `.taskflow/roadmap/*.toml` files directly for task, milestone, metadata, status, design, archive, or execution changes. Use `taskflow-ai` commands so IDs, timestamps, links, and generated roadmaps stay consistent.

## Standard Workflow
1. **Discover**: 
   - Run `taskflow-ai next` to find the highest priority task and its associated designs.
   - Run `taskflow-ai task templates list` to see available task templates if creating a new task. Create tasks with `taskflow-ai task add <Title> -T <template_name>`.
2. **Task Templates & Requirements**:
   - If a task uses a template, run `taskflow-ai task templates show <template_name>` to view required metadata and designs.
   - Satisfy required metadata using `taskflow-ai task meta set <TF_ID> <key> <value>`.
3. **Research & Design**:
   - Read the linked HLD/LLDs.
   - Use `taskflow-ai design templates show <type>` to view required headers for a design type.
   - Run `taskflow-ai design init <type> <Title> --milestone <M_ID> [--task <TF_ID>]` to create required designs.
   - Populate the scaffolded Markdown file.
4. **Execute**:
   - Start: `taskflow-ai task execute start <TF_ID> --agent <Agent_Name>`.
   - Implement the change.
   - Validate: `taskflow-ai task validate <TF_ID>`. This MUST pass before completing the task to ensure all template requirements are met.
   - Complete: `taskflow-ai task execute complete <TF_ID> --outcome success --log "Summary of work"`.
5. **Sync**: The roadmap usually syncs automatically, but you can run `taskflow-ai sync` to be sure.

## Command Reference
- `taskflow-ai init <PROJECT_NAME>`: Initialize a TaskFlowAI project.
- `taskflow-ai task add <Title> [--task-type <type>] [--milestone <M_ID>] [--parent <TF_ID>] [--template <name>]`: Create backlog, milestone, or child tasks.
- `taskflow-ai task list [--milestone <M_ID>]`: List tasks.
- `taskflow-ai task show <TF_ID>`: Inspect a task, including metadata, designs, and execution history.
- `taskflow-ai next`: Select the next recommended task.
- `taskflow-ai task status <TF_ID> <status>`: Change task status.
- `taskflow-ai task execute start|complete <TF_ID>`: Track task execution.
- `taskflow-ai task meta set <TF_ID> <key> <value>`: Set task metadata.
- `taskflow-ai milestone create|edit|list|archive ...`: Manage milestones and milestone priority/archiving.
- `taskflow-ai task move --milestone <M_ID> <TF_ID>`: Move a task between backlog and milestones.
- `taskflow-ai task delete <TF_ID>`: Delete a task through the storage layer.
- `taskflow-ai task edit <TF_ID>`: Open the supported interactive task editor when a command-specific update is unavailable.
- `taskflow-ai design init|templates ...`: Create or inspect design documents and templates.
- `taskflow-ai task templates list|show <name>`: Inspect task templates.
- `taskflow-ai task validate <TF_ID>`: Check template and design requirements.
- `taskflow-ai sync`: Regenerate roadmap Markdown from TOML state.
- `taskflow-ai dashboard`: Show the dashboard view.
- `taskflow-ai config <key> [value]`: Read or update project configuration.
- `taskflow-ai skill view|install <provider>`: View or install these agent instructions.
- `taskflow-ai completions <shell>`: Generate shell completions.

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
            let path = skill_path_for_provider(&provider, Path::new(&home))?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {:?}", parent))?;
            }

            fs::write(&path, TASKFLOW_SKILL_PROMPT)
                .with_context(|| format!("Failed to write skill to: {:?}", path))?;
            println!(
                "Successfully installed TaskFlowAI skill to {}",
                path.display()
            );
        }
    }
    Ok(())
}

const SUPPORTED_PROVIDERS: &str = "claude, gemini, codex, dirac, opencode, agents";

fn skill_path_for_provider(provider: &str, home: &Path) -> Result<PathBuf> {
    let path = match provider.to_lowercase().as_str() {
        "claude" => home.join(".claude/skills/taskflow/SKILL.md"),
        "gemini" => home.join(".gemini/skills/taskflow/SKILL.md"),
        "codex" => home.join(".codex/skills/taskflow/SKILL.md"),
        "dirac" => home.join(".dirac/skills/taskflow/SKILL.md"),
        "opencode" => home.join(".config/opencode/skill/taskflow/SKILL.md"),
        "agents" => home.join(".agents/skills/taskflow/SKILL.md"),
        _ => bail!(
            "Unsupported provider: {}. Supported providers are: {}",
            provider,
            SUPPORTED_PROVIDERS
        ),
    };

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::skill_path_for_provider;
    use std::path::Path;

    #[test]
    fn agents_provider_installs_to_general_agents_skill_directory() {
        assert_eq!(
            skill_path_for_provider("agents", Path::new("/tmp/home")).unwrap(),
            Path::new("/tmp/home/.agents/skills/taskflow/SKILL.md")
        );
    }

    #[test]
    fn provider_matching_is_case_insensitive() {
        assert_eq!(
            skill_path_for_provider("AGENTS", Path::new("/tmp/home")).unwrap(),
            Path::new("/tmp/home/.agents/skills/taskflow/SKILL.md")
        );
    }

    #[test]
    fn unsupported_provider_error_lists_agents() {
        let error = skill_path_for_provider("unknown", Path::new("/tmp/home")).unwrap_err();
        assert!(error.to_string().contains("agents"));
    }
}
