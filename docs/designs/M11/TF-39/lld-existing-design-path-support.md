# Low-Level Design: Existing Design Path Support

## 1. Objective
Allow `taskflow-ai design init --path` to register an existing Markdown design document without trying to create a directory at that file path. This preserves the current custom-directory behavior while supporting prewritten design files.

## 2. Architecture
`src/commands/design.rs` remains the single owner of design path resolution, scaffolding, and metadata registration. The CLI shape does not change.

## 3. Implementation Details
- Treat `--path` values ending in `.md` as explicit document paths.
- Treat all other `--path` values as directories and append the generated design filename, preserving existing behavior.
- If the resolved document already exists, skip scaffolding and only register metadata.
- If the resolved document does not exist, scaffold it from the design template.

## 4. Verification Plan
- Run `cargo test`.
- Run `cargo run -- design init lld "Existing File Smoke" --milestone M11 --task TF-39 --path docs/designs/M11/TF-39/existing-file-smoke.md` against a prewritten file and confirm it registers without `File exists`.
