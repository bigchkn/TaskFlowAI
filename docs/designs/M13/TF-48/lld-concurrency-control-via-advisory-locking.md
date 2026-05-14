# Low-Level Design: Concurrency Control via Advisory Locking

## 1. Objective
Prevent data corruption and race conditions when multiple instances of `taskflow-ai` are run concurrently against the same project.

## 2. Architecture
We will implement **Advisory Locking** using the `fd-lock` crate. 
- A lock will be acquired on `.taskflow/roadmap/index.toml` whenever a command needs to read or write project state.
- The lock will be an exclusive lock for operations that modify state.
- Because TaskFlowAI uses a read-modify-write pattern for fragments, a global project lock is the most reliable way to ensure consistency.

## 3. Implementation Details
1. **Add Dependency**: Add `fd-lock = "4.0"` to `Cargo.toml`.
2. **Update Storage Trait**: Add a mechanism to the `Storage` trait to acquire a long-lived lock if necessary, or wrap existing methods in locking logic.
3. **FileStorage Implementation**: 
   - Open `index.toml` with appropriate permissions.
   - Use `fd_lock::RwLock` to manage access.
   - For simplicity and absolute safety, most commands will acquire an exclusive lock at the start of their execution if they are stateful.
4. **Error Handling**: If a lock cannot be acquired immediately, the CLI should either wait or exit with a friendly message: "Project is currently locked by another process."

## 4. Verification Plan
1. **Automated Race Test**: Create a script that launches 10 concurrent `taskflow-ai add` commands.
2. **Integrity Check**: Verify that all 10 tasks are correctly added to the TOML and the Roadmap Markdown without data loss or corruption.
3. **Stale Lock Check**: Force-kill a process holding a lock and verify that subsequent commands can still run (proving OS-level cleanup).
