# Low-Level Design: Refactor main.rs

## 1. Objective
Modularize `src/main.rs` by moving the implementation of CLI commands into a dedicated `commands` module. This will improve readability, maintainability, and allow for easier testing of individual command logic.

## 2. Architecture
1.  **New Module**: Create `src/commands/mod.rs` (or `src/commands.rs`).
2.  **Dispatcher**: `main.rs` will keep the `Cli` and `Commands` definitions and the top-level `match` block, but will delegate work to functions in the `commands` module.
3.  **Command Handlers**: Each subcommand (e.g., `add`, `list`, `move`) will have a corresponding function:
    -   `commands::add(...)`
    -   `commands::list(...)`
    -   `commands::move_task(...)`
    -   etc.

## 3. Implementation Details
-   Move constants like `TASKFLOW_SKILL_PROMPT` to the `commands` module.
-   Pass `storage` and relevant arguments to each handler.
-   Ensure `Result` types are handled consistently.

## 4. Verification Plan
1.  Refactor one command at a time and verify it still works.
2.  Run `cargo build` to ensure no regressions.
3.  Run `taskflow-ai dashboard` and `taskflow-ai next` to verify core functionality.
