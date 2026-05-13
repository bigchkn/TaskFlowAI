# Low-Level Design: Support agents skill install target

## 1. Objective
Add support for `taskflow-ai skill install agents` so the TaskFlowAI skill can be installed into the general agents skill directory at `~/.agents/skills/...`.

The existing skill install command should keep its current provider behavior while accepting `agents` as a first-class install target.

## 2. Architecture
The change belongs in the skill command provider routing. The command should map each supported provider to a destination root and then reuse the existing skill file generation/copy path.

The `agents` provider should resolve to the user's home directory and install under `.agents/skills/`, matching the general agents convention rather than a Codex-specific skill location.

## 3. Implementation Details
1. Extend the provider parsing or match logic for `taskflow-ai skill install <provider>` to accept `agents`.
2. Add destination resolution for `agents` to `~/.agents/skills/<taskflow skill directory>`.
3. Ensure parent directories are created before writing files.
4. Keep existing supported providers and `skill view` behavior unchanged.
5. Update command help, docs, or tests that enumerate supported providers.

## 4. Verification Plan
Run the focused skill command tests and add coverage for:

1. `taskflow-ai skill install agents` writes the expected files under a temporary home directory equivalent of `~/.agents/skills/...`.
2. Existing install providers still resolve to their current destinations.
3. Unsupported provider names still fail with a clear error.
