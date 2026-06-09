# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0-beta.1] - 2026-06-07

### Added
- **Full Parity with gitflow-avh**: 
  - `gitflow version`: Outputs the current version.
  - `gitflow log`: Displays logs matching AVH functionality.
  - `gitflow config`: View, set, and list gitflow configurations natively.
- **Feature Branches**: Added `track`, `publish`, `diff`, `rebase`, `checkout` commands.
- **Bugfix Branches**: Added `publish`, `track`, `diff`, `rebase`, `checkout`, `pull`, `delete`, and `list` commands.
- **Hotfix Branches**: Added `publish`, `track`, `delete`, `list`, and `rebase` commands.
- **Release Branches**: Added `track`, `list`, and `rebase` commands.
- **Support Branches**: Added `list` command.
- Native Git command passthrough to execute pagers and interactive rebases smoothly.
- Comprehensive GitHub Actions CI/CD Pipeline.

### Changed
- Refactored `run_git` usages for long-running processes (like `diff` and `log`) to bind directly to terminal output streams.

### Fixed
- Addressed multiple linter warnings from `cargo clippy`.
