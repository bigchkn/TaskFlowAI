# Low-Level Design: Global Installer and Build Hook

## 1. Objective
Enable global access to the `taskflow-ai` command from any directory and provide a mechanism to automatically update the installed binary after successful builds.

## 2. Architecture
1. **Installation Mechanism**: Use `cargo install --path .` to build and install the binary to the standard Cargo bin directory (`~/.cargo/bin`).
2. **Convenience Script**: An `install.sh` script in the root directory will wrap the build and installation process.
3. **Build Hook**: The script can be executed manually or as part of a CI/CD or local watch process to ensure the global binary stays in sync with the source code.

## 3. Implementation Details
- **`install.sh` Script**:
    - Checks for `cargo` availability.
    - Runs `cargo build --release`.
    - Installs the binary using `cargo install --path .`.
    - Verifies the installation by running `taskflow-ai --version` (if implemented, else just check existence).

## 4. Verification Plan
1. Run `./install.sh`.
2. Open a new terminal or source the shell config.
3. Run `taskflow-ai dashboard` from a directory outside the project root.
4. Verify the output matches the project state.
