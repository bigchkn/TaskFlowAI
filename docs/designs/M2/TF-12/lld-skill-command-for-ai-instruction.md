# Low-Level Design: Skill Command for AI Instruction

## 1. Objective
Provide a dedicated command, `taskflow-ai skill`, that outputs a comprehensive system prompt or "skill" definition. This instruction set allows any AI agent to immediately understand the TaskFlowAI directory structure, command set, and the "ideal" workflow (Research -> Design -> Execute -> Sync).

## 2. Architecture
The command will be a simple output generator. It will print a Markdown-formatted block of text containing:
1. **Core Philosophy**: Metadata vs. Documentation.
2. **Directory Map**: `.taskflow/`, `roadmap/`, `docs/designs/`.
3. **Workflow Steps**:
    - Use `next` to find work.
    - Use `design init` before implementation.
    - Use `execute start/complete` for tracking.
    - Use `sync` to update roadmaps.
4. **Command Reference**: A concise summary of CLI usage.

## 3. Implementation Details
- **Command**: `taskflow-ai skill`
- **Output**: A static (but potentially templated in the future) Markdown string optimized for LLM system prompts.
- **Integration**: Agents can run this command at the start of a session to "hydrate" their context with TaskFlowAI-specific knowledge.

## 4. Verification Plan
1. Run `taskflow-ai skill`.
2. Verify the output is valid Markdown.
3. Verify all core commands (`add`, `next`, `design`, `execute`, `sync`) are mentioned and explained.
