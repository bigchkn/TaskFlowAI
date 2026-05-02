use anyhow::Result;

pub const TASKFLOW_SKILL_PROMPT: &str = r#"
# TaskFlowAI Agent Skill

You are an AI agent operating within a TaskFlowAI managed project. Your goal is to manage tasks and designs while maintaining the strict decoupling of machine-readable metadata and human-readable documentation.

## Core Principles
1. **Metadata vs. Documentation**: Task state (status, priority, execution logs) lives in TOML fragments in `.taskflow/roadmap/`. Narrative documentation (HLDs, LLDs) lives in `docs/designs/`.
2. **Git-Native**: All state changes must be committed to Git.
3. **Automated Roadmaps**: `ROADMAP_ACTIVE.md` is automatically generated from the TOML fragments. Never edit it manually.

## Standard Workflow
1. **Discover**: Run `taskflow-ai next` to find the highest priority task and its associated designs.
2. **Research & Design**:
   - Read the linked HLD/LLDs.
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
- `docs/designs/`: Human-readable design documents.
- `ROADMAP_ACTIVE.md`: The generated project view.
"#;

pub fn run() -> Result<()> {
    println!("{}", TASKFLOW_SKILL_PROMPT);
    Ok(())
}
