# Project Roadmap: TaskFlowAI Core



## Active Milestones

### Design Phase (M1)
**Status:** Todo

**Designs:**
- [Hld] docs/designs/M1/hld-architecture-overview.md (`Approved`)

- [x] **TF-1**: Implement core parser(`Done`)
  - [Lld] docs/designs/M1/TF-1/lld-parser-implementation.md (`Draft`)
- [x] **TF-2**: Initialize project and define fragmented storage models(`Done`)
- [x] **TF-3**: Implement archiving and automated roadmap generation(`Done`)
- [x] **TF-4**: Implement Multi-Level Design Management (HLD/LLD)(`Done`)
- [x] **TF-5**: Implement template-based design document validation(`Done`)

### Core Commands & Refinement (M2)
**Status:** Todo

- [x] **TF-6**: Implement move command (Backlog to Milestone)(`Done`)
  - [Lld] docs/designs/M2/TF-6/lld-move-command-implementation.md (`Draft`)
- [x] **TF-7**: Implement edit command (Interactive TOML editing)(`Done`)
  - [Lld] docs/designs/M2/TF-7/lld-edit-command-implementation.md (`Draft`)
- [x] **TF-10**: Implement next command for task prioritization(`Done`)
  - [Lld] docs/designs/M2/TF-10/lld-next-command-implementation.md (`Draft`)
- [x] **TF-11**: Create global installer and build hook(`Done`)
  - [Lld] docs/designs/M2/TF-11/lld-global-installer-and-build-hook.md (`Draft`)
- [x] **TF-12**: Implement 'skill' command for AI workflow instruction(`Done`)
  - [Lld] docs/designs/M2/TF-12/lld-skill-command-for-ai-instruction.md (`Draft`)

### Advanced Features (M3)
**Status:** Todo

- [x] **TF-8**: Implement hierarchical subtask support (--parent flag)(`Done`)
  - [Lld] docs/designs/M3/TF-8/lld-hierarchical-subtask-support.md (`Draft`)
- [x] **TF-13**: Architectural Refactoring(`Done`)
  - [ ] **TF-14**: Refactor storage layer (Parent: TF-13)(`Backlog`)
  - [x] **TF-15**: Refactor main.rs into modular command handlers (Parent: TF-13)(`Done`)
    - [Lld] docs/designs/M3/TF-15/lld-refactor-main.rs.md (`Draft`)

### Self-Hosting & Validation (M4)
**Status:** Todo

- [x] **TF-9**: Dogfood TaskFlowAI on its own repository development(`Done`)

### Provider Skills Integration (M5)
**Status:** Todo

- [x] **TF-17**: Refine skill command with view and install subcommands(`Done`)
  - [Lld] docs/designs/M5/TF-17/lld-skill-command-refinement.md (`Approved`)

### Milestone Priority Support (M6)
**Priority:** 10
**Status:** Todo

**Designs:**
- [Lld] docs/designs/M6/lld-milestone-priority-support.md (`Draft`)

- [x] **TF-18**: Update MilestoneMetadata model to include priority(`Done`)
- [x] **TF-19**: Update milestone create and add milestone edit command(`Done`)
- [x] **TF-20**: Update next command to respect milestone priority(`Done`)
- [x] **TF-21**: Update roadmap generation to include priority(`Done`)
- [x] **TF-22**: Final validation and sync(`Done`)

### Improve Next Command Display (M8)
**Priority:** 255
**Status:** Todo

**Designs:**
- [Lld] docs/designs/M8/lld-improve-next-command-output.md (`Draft`)

- [x] **TF-23**: Include relevant design docs in 'next' command output(`Done`)
  - [Lld] docs/designs/M8/TF-23/lld-implementation-details-for-tf-23.md (`Draft`)

## Backlog

_Backlog is empty._

