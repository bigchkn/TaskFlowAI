# Low-Level Design: Update taskflow-ai skill with Task Template instructions

## 1. Objective
Update the `TASKFLOW_SKILL_PROMPT` inside `src/commands/skill.rs` to provide AI agents with explicit instructions on how to use, apply, and fulfill Task Templates within the TaskFlowAI ecosystem. This ensures agents know how to look up template requirements and satisfy them before completing a task.

## 2. Architecture
- **Component**: `src/commands/skill.rs`
- **Constant**: `TASKFLOW_SKILL_PROMPT`
- The change is localized to the string constant containing the skill prompt. No core logic changes are required.

## 3. Implementation Details
The `TASKFLOW_SKILL_PROMPT` will be updated to include a specific workflow or section for Task Templates.

Modifications to the prompt text:
1. In the **Discover** section, suggest using `taskflow-ai templates list` when planning to add a new task, and using the `-T <template_name>` flag with `taskflow-ai add`.
2. Add a new step in the **Execute** section or a dedicated **Task Templates** section that instructs the agent to:
   - Run `taskflow-ai templates show <template_name>` if a task was created with a template (or if it has `template = "..."` in its metadata).
   - Use `taskflow-ai meta set <TF_ID> <key> <value>` to fulfill any required metadata fields defined by the template.
   - Run `taskflow-ai design init <type> ...` to create any designs required by the template.
3. Emphasize that `taskflow-ai validate <TF_ID>` must be run and must pass before marking the task as complete, as the validation step explicitly checks template requirements.

## 4. Verification Plan
- **View Prompt**: Run `cargo run -- skill view` and verify the output contains the new Task Template instructions.
- **Validation Check**: The updated prompt should explicitly mention `taskflow-ai templates list`, `taskflow-ai templates show <name>`, and `taskflow-ai meta set`.
