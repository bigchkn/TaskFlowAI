# Low-Level Design: Milestone Priority Support

## 1. Objective
Enhance the TaskFlow system by introducing priority levels to milestones. This will allow the `next` command to intelligently suggest tasks from higher-priority milestones first, enabling better guidance of execution flow.

## 2. Architecture
The `MilestoneMetadata` struct in `model.rs` will be extended with a `priority` field. The `next` command in `commands/next.rs` will be updated to sort milestones by this priority before task selection. New CLI arguments will be added to `main.rs` to allow setting and editing this priority.

## 3. Implementation Details

### Data Model Update
- **File:** `src/model.rs`
- **Action:** Add `#[serde(default)] pub priority: u8` to the `MilestoneMetadata` struct. 

### CLI Command Updates
- **File:** `src/main.rs`
- **Action:** 
  - Update `MilestoneCommands::Create` to include an `#[arg(short, long, default_value_t = 0)] priority: u8` argument.
  - Add a new enum variant `MilestoneCommands::Edit { id: String, #[arg(short, long)] priority: Option<u8> }`.
- **Action:** Update the match statement in `main()` to route `MilestoneCommands::Edit` to `commands::milestone_edit`.

### Command Implementation Updates
- **File:** `src/commands/milestone.rs`
- **Action:** 
  - Update `create` function signature to accept `priority: u8`.
  - Implement a new `pub fn edit<S: Storage>(storage: &S, id: String, priority: Option<u8>) -> Result<()>` function.

### Next Command Update
- **File:** `src/commands/next.rs`
- **Action:** Sort milestones by priority descending before searching for pending tasks.

### Roadmap Generation Update
- **File:** `src/roadmap.rs`
- **Action:** Output the priority in `ROADMAP_ACTIVE.md` and `ROADMAP_ARCHIVE.md`.

## 4. Verification Plan
1. **Model:** The project compiles successfully. Existing `.taskflow/roadmap/index.toml` files load without errors.
2. **Create:** Creating a new milestone `M7` with `--priority 10` saves correctly to the index.
3. **Edit:** Using `taskflow-ai milestone edit M6 --priority 5` updates the index.
4. **Roadmap:** Running `taskflow-ai sync` shows `Priority: 10` and `Priority: 5` under the respective milestones in `ROADMAP_ACTIVE.md`.
5. **Next Task:** Moving tasks to `M6` (priority 5) and `M7` (priority 10). The `taskflow-ai next` command should suggest tasks from `M7` before `M6`.
