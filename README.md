# Gitflow Rust

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Gitflow Rust** is a high-performance, type-safe implementation of the Gitflow AVH branching model, rewritten from the ground up in Rust. It provides a robust CLI interface to manage your development workflow with speed and reliability.

## Key Features

- **Zero-Cost Abstractions:** Leverages Rust's performance to provide a near-instant CLI experience.
- **Type-Safe CLI:** Built with `clap` for rigorous argument validation and clear error messaging.
- **Native Shell Completions:** Built-in generation for Bash, Zsh, Fish, and PowerShell to speed up your daily workflow.
- **Safety First:** Prevents logical errors (like simultaneous rebase and squash) at the parsing stage.

---

## Installation

Ensure you have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed.

```bash
git clone https://github.com/youruser/gitflow-rust.git
cd gitflow-rust
cargo build --release
```

The compiled binary will be located at `target/release/gitflow`.

### Making it Global

**Option 1: Using Cargo (Recommended)**

Automatically builds and moves the binary to your Cargo bin path:

```bash
cargo install --path .
```

**Option 2: Manual Move (Linux/macOS)**

```bash
sudo cp target/release/gitflow /usr/local/bin/
```

---

## Usage

### Initialization

Set up your repository with the Gitflow structure:

```bash
gitflow init
```

### Feature Management

```bash
# Start a new feature branch from develop
gitflow feature start my-awesome-feature

# Finish the feature (merge into develop and delete the branch)
gitflow feature finish my-awesome-feature
```

### Release Management

```bash
# Start a new release
gitflow release start 1.0.0

# Finish the release (merge into main and develop, then tag)
gitflow release finish 1.0.0
```

### Shell Completions

**Bash (`~/.bashrc`):**
```bash
source <(gitflow completions bash)
```

**Zsh (`~/.zshrc`):**
```bash
source <(gitflow completions zsh)
```

---

## Architecture

- `src/parser/models.rs` — All `clap` structures, enums, and command definitions.
- `src/parser/service.rs` — Implements the `Execute` trait, housing the logic for each command.
- `src/main.rs` — Entry point that orchestrates parsing and execution.

---

## Roadmap

- [x] Full CLI argument parsing and validation.
- [x] Shell completion generation.
- [ ] Integration with `git2-rs` for native Git operations.
- [ ] Configuration file support (`.gitflow`).
- [ ] GPG signing for release tags.

---

## Contributing

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/amazing-feature`).
3. Commit your changes (`git commit -m 'feat: add some amazing feature'`).
4. Push to the branch (`git push origin feature/amazing-feature`).
5. Open a Pull Request.

---

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.
