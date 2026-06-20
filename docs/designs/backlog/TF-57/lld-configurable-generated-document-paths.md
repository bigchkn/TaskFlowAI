# Low-Level Design: Configurable Generated Document Paths

## 1. Objective
Make generated human-readable document locations configurable while preserving the existing default layout when no config values are present.

## 2. Architecture
Project-level config already lives in `Project.config` and is managed by `taskflow-ai config`. Add a small path helper module that resolves documented config keys with hardcoded defaults:

- `document_dir` defaults to `docs/designs`
- `roadmap_active_path` defaults to `ROADMAP_ACTIVE.md`
- `roadmap_archive_path` defaults to `ROADMAP_ARCHIVE.md`

Commands continue to store paths relative to the project when relative config values are used. Absolute roadmap paths are accepted for output generation.

## 3. Implementation Details
`design init` uses `document_dir` when constructing the default scaffold path. Explicit `--path` values still take precedence.

`roadmap::generate_roadmaps` writes the active and archive Markdown files to the configured paths, creating parent directories when needed. Missing or blank config values fall back to current hardcoded locations.

README documents the keys and examples for setting them.

## 4. Verification Plan
Run `cargo test` to cover default and configured path resolution. Run `taskflow-ai task validate TF-57` after implementation to confirm the task template requirements remain satisfied.
