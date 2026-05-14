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

### Self-Hosting & Validation (M4)
**Status:** Todo

- [x] **TF-9**: Dogfood TaskFlowAI on its own repository development(`Done`)

### Provider Skills Integration (M5)
**Status:** Todo

- [x] **TF-17**: Refine skill command with view and install subcommands(`Done`)
  - [Lld] docs/designs/M5/TF-17/lld-skill-command-refinement.md (`Approved`)
- [x] **TF-43**: Support agents skill install target for taskflow-ai skill install(`Done`)
  - [Lld] docs/designs/M5/TF-43/lld-support-agents-skill-install-target.md (`Approved`)
  - [x] **TF-44**: Write LLD (Parent: TF-43)(`Done`)
  - [x] **TF-45**: Implementation (Parent: TF-43)(`Done`)

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

### Task Templates & Workflow Enforcement (M9)
**Priority:** 255
**Status:** Todo

**Designs:**
- [Lld] docs/designs/M9/lld-task-templates-implementation.md (`Draft`)

- [x] **TF-24**: Update taskflow-ai skill with Task Template instructions(`Done`)
  - [Lld] docs/designs/M9/TF-24/lld-update-taskflow-ai-skill-with-task-template-instructions.md (`Approved`)
  - [x] **TF-25**: Write LLD (Parent: TF-24)(`Done`)
  - [x] **TF-26**: Implementation (Parent: TF-24)(`Done`)
- [x] **TF-27**: Implement 'templates list' sub-command(`Done`)
  - [Lld] docs/designs/M9/TF-27/lld-templates-command-implementation.md (`Approved`)
  - [x] **TF-28**: Write LLD (Parent: TF-27)(`Done`)
  - [x] **TF-29**: Implementation (Parent: TF-27)(`Done`)

### Design Template CLI Support (M10)
**Priority:** 255
**Status:** Todo

**Designs:**
- [Rfc] docs/research/rfc-test-research.md (`Draft`)

- [x] **TF-30**: Design: Design Template CLI Command(`Done`)
- [x] **TF-31**: Implementation: Design Template CLI Command(`Done`)
- [x] **TF-32**: Update: taskflow-ai Skill Documentation(`Done`)
- [x] **TF-33**: Support custom paths in 'design init' and add Research template(`Done`)
  - [Lld] docs/designs/M10/TF-33/lld-support-custom-paths-in-design-init-and-research-template.md (`Approved`)
  - [x] **TF-34**: Write LLD (Parent: TF-33)(`Done`)
  - [x] **TF-35**: Implementation (Parent: TF-33)(`Done`)

### Existing Design Path Support (M11)
**Priority:** 255
**Status:** Todo

- [x] **TF-39**: Allow design init to register existing document paths(`Done`)
  - [Lld] docs/designs/M11/TF-39/lld-existing-design-path-support.md (`Approved`)
  - [x] **TF-40**: Write LLD (Parent: TF-39)(`Done`)
  - [x] **TF-41**: Implementation (Parent: TF-39)(`Done`)

## Backlog

- [x] **TF-36**: Test Research Task(`Done`)
  - [Rfc] docs/research/rfc-test-research-task-output.md (`Draft`)
  - [x] **TF-37**: Initial Research & Discovery (Parent: TF-36)(`Done`)
  - [x] **TF-38**: Document Findings (Parent: TF-36)(`Done`)
- [x] **TF-42**: Update skill prompt to prefer taskflow-ai commands(`Done`)
