# Gitflow Rust

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![CI](https://github.com/ImKairat-Rust-Lab/gitflow/actions/workflows/ci.yml/badge.svg)](https://github.com/ImKairat-Rust-Lab/gitflow/actions/workflows/ci.yml)
[![Version](https://img.shields.io/github/v/tag/ImKairat-Rust-Lab/gitflow?color=blue&label=version)](https://github.com/ImKairat-Rust-Lab/gitflow/releases)


**Gitflow Rust** is a high-performance, type-safe implementation of the Gitflow AVH branching model, rewritten from the ground up in Rust. It provides a robust CLI interface to manage your development workflow with speed and reliability.

> **Note:** This project is heavily inspired by and aims to provide full feature parity with [petervanderdoes/gitflow-avh](https://github.com/petervanderdoes/gitflow-avh).

## Key Features

- **Zero-Cost Abstractions:** Leverages Rust's performance to provide a near-instant CLI experience.
- **Type-Safe CLI:** Built with `clap` for rigorous argument validation and clear error messaging.
- **Native Shell Completions:** Built-in generation for Bash, Zsh, Fish, and PowerShell to speed up your daily workflow.
- **Safety First:** Prevents logical errors (like simultaneous rebase and squash) at the parsing stage.

---

## Installation

Ensure you have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed.

### Building from Source

```bash
git clone https://github.com/ImKairat-Rust-Lab/gitflow.git
cd gitflow
cargo build --release
```

The compiled binary will be located at `target/release/gitflow`.

### Making it Global

**Option 1: Using Cargo (Recommended)**

Automatically builds and moves the binary to your Cargo bin path:

```bash
cargo install --path .
```

**Option 2: Arch Linux (using `makepkg`)**

If you are on Arch Linux, you can build and install it natively using the provided `PKGBUILD`:

```bash
makepkg -si
```

**Option 3: Manual Move (Linux/macOS)**

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

- `src/cli.rs` — All `clap` structures, enums, and top-level command routing.
- `src/commands/*.rs` — Implements the `Execute` trait, housing the logic for each subcommand (`feature`, `release`, etc.).
- `src/git/` — Core wrappers around native Git commands and hooks execution.
- `src/main.rs` — Entry point that orchestrates parsing and execution.

---

## Roadmap

- [x] Full CLI argument parsing and validation.
- [x] Shell completion generation.
- [x] Integration with `git2-rs` for native Git operations.
- [x] Configuration file support (`.gitflow`).
- [x] GPG signing for release tags.

---

## Contributing

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/amazing-feature`).
3. Commit your changes (`git commit -m 'feat: add some amazing feature'`).
4. Push to the branch (`git push origin feature/amazing-feature`).
5. Open a Pull Request.

---

## License

Distributed under the GPLv3 License. See [LICENSE](LICENSE) for more information.
