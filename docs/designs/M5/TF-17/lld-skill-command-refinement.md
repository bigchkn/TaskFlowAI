# Low-Level Design: TF-17 Refine Skill Command

## 1. Objective
Refactor the `taskflow-ai skill` command to support subcommands: `view` (current behavior) and `install <provider>` (writes the skill instructions to the provider's specific configuration file in the project root). Supported providers: `claude`, `gemini`, `codex`, `dirac`, and `opencode`.

## 2. Architecture
- **CLI Parser (`src/main.rs`)**: Update the `Commands::Skill` variant to accept subcommands.
- **Command Handler (`src/commands/skill.rs`)**: Implement logic to handle the `view` and `install` actions, including file I/O for the supported providers.

## 3. Implementation Details

### 3.1 CLI Changes (`src/main.rs`)
Update the Clap definition for the `skill` command:
```rust
#[derive(Subcommand, Debug)]
pub enum Commands {
    // ... other commands
    /// Manage the TaskFlowAI agent skill
    Skill {
        #[command(subcommand)]
        command: Option<SkillCommands>,
    },
}

#[derive(Subcommand, Debug)]
pub enum SkillCommands {
    /// View the skill prompt (default)
    View,
    /// Install the skill for a specific AI provider
    Install {
        /// The AI provider (claude, gemini, codex, dirac, opencode)
        provider: String,
    },
}
```
Update the routing logic:
```rust
Commands::Skill { command } => commands::skill(command),
```

### 3.2 Command Logic (`src/commands/skill.rs`)
Update the function signature and add the routing logic:

```rust
use std::fs;
use crate::SkillCommands;
use anyhow::{Result, bail};

// ... TASKFLOW_SKILL_PROMPT ...

pub fn run(command: Option<SkillCommands>) -> Result<()> {
    let cmd = command.unwrap_or(SkillCommands::View);

    match cmd {
        SkillCommands::View => {
            println!("{}", TASKFLOW_SKILL_PROMPT);
        }
        SkillCommands::Install { provider } => {
            let file_path = match provider.to_lowercase().as_str() {
                "claude" => "CLAUDE.md",
                "gemini" => "GEMINI.md",
                "codex" => ".cursorrules",
                "dirac" => "AGENTS.md",
                "opencode" => "AGENTS.md",
                _ => bail!("Unsupported provider: {}. Supported providers are: claude, gemini, codex, dirac, opencode", provider),
            };

            fs::write(file_path, TASKFLOW_SKILL_PROMPT)?;
            println!("Successfully installed TaskFlowAI skill to {}", file_path);
        }
    }
    Ok(())
}
```

## 4. Verification Plan
- **View Command**: Run `cargo run -- skill` and `cargo run -- skill view`. Both should print the prompt.
- **Install Command**:
  - Run `cargo run -- skill install claude` and verify `CLAUDE.md` is created/updated with the prompt.
  - Run `cargo run -- skill install gemini` and verify `GEMINI.md` is created/updated.
  - Run `cargo run -- skill install codex` and verify `.cursorrules` is created/updated.
  - Run `cargo run -- skill install dirac` and verify `AGENTS.md` is created/updated.
  - Run `cargo run -- skill install opencode` and verify `AGENTS.md` is created/updated.
- **Unsupported Provider**: Run `cargo run -- skill install unknown` and ensure it returns a clear error message.
