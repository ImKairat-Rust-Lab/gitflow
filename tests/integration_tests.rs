// Copyright (C) 2026 Kairat Kubanychbek uulu <https://github.com/ImKairat>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::process::Command as StdCommand;
use tempfile::tempdir;

fn setup_repo(dir: &std::path::Path) {
    StdCommand::new("git")
        .arg("init")
        .arg("-b")
        .arg("main")
        .current_dir(dir)
        .output()
        .expect("Failed to init repo");

    StdCommand::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(dir)
        .output()
        .expect("Failed to set email");

    StdCommand::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(dir)
        .output()
        .expect("Failed to set name");

    // Create an initial commit on main
    fs::write(dir.join("README.md"), "test").expect("Failed to write file");
    StdCommand::new("git")
        .args(&["add", "."])
        .current_dir(dir)
        .output()
        .expect("Failed to add");
    StdCommand::new("git")
        .args(&["commit", "-m", "Initial commit"])
        .current_dir(dir)
        .output()
        .expect("Failed to commit");
}

fn get_current_branch(dir: &std::path::Path) -> String {
    let output = StdCommand::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(dir)
        .output()
        .expect("Failed to get branch");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn branch_exists(dir: &std::path::Path, branch: &str) -> bool {
    let output = StdCommand::new("git")
        .args(&["branch", "--list", branch])
        .current_dir(dir)
        .output()
        .expect("Failed to list branches");
    !String::from_utf8_lossy(&output.stdout).trim().is_empty()
}

fn tag_exists(dir: &std::path::Path, tag: &str) -> bool {
    let output = StdCommand::new("git")
        .args(&["tag", "-l", tag])
        .current_dir(dir)
        .output()
        .expect("Failed to list tags");
    !String::from_utf8_lossy(&output.stdout).trim().is_empty()
}

#[test]
fn test_init_default() {
    let dir = tempdir().expect("Failed to create temp dir");
    setup_repo(dir.path());

    let mut cmd = Command::cargo_bin("gitflow").unwrap();
    cmd.current_dir(dir.path())
        .arg("init")
        .arg("-d")
        .assert()
        .success();

    assert_eq!(get_current_branch(dir.path()), "develop");
    assert!(branch_exists(dir.path(), "main"));
    assert!(branch_exists(dir.path(), "develop"));
}

#[test]
fn test_feature_workflow() {
    let dir = tempdir().expect("Failed to create temp dir");
    setup_repo(dir.path());

    // Init
    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["init", "-d"])
        .assert()
        .success();

    // Feature Start
    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["feature", "start", "my-feature"])
        .assert()
        .success();
    assert_eq!(get_current_branch(dir.path()), "feature/my-feature");

    // Add some changes
    fs::write(dir.path().join("feature.txt"), "feat").unwrap();
    StdCommand::new("git")
        .args(&["add", "."])
        .current_dir(dir.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(&["commit", "-m", "feat commit"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    // Feature Finish
    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["feature", "finish", "my-feature"])
        .assert()
        .success();

    assert_eq!(get_current_branch(dir.path()), "develop");
    assert!(!branch_exists(dir.path(), "feature/my-feature"));
}

#[test]
fn test_release_workflow() {
    let dir = tempdir().expect("Failed to create temp dir");
    setup_repo(dir.path());

    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["init", "-d"])
        .assert()
        .success();

    // Release Start
    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["release", "start", "1.1.0"])
        .assert()
        .success();
    assert_eq!(get_current_branch(dir.path()), "release/1.1.0");

    // Release Finish
    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["release", "finish", "1.1.0"])
        .assert()
        .success();

    assert_eq!(get_current_branch(dir.path()), "develop");
    assert!(tag_exists(dir.path(), "v1.1.0"));
    assert!(!branch_exists(dir.path(), "release/1.1.0"));
}

#[test]
fn test_hotfix_workflow() {
    let dir = tempdir().expect("Failed to create temp dir");
    setup_repo(dir.path());

    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["init", "-d"])
        .assert()
        .success();

    // Go back to main for a moment to simulate being on develop
    StdCommand::new("git")
        .args(&["checkout", "develop"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    // Hotfix Start
    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["hotfix", "start", "1.1.1"])
        .assert()
        .success();
    assert_eq!(get_current_branch(dir.path()), "hotfix/1.1.1");

    // Hotfix Finish
    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["hotfix", "finish", "1.1.1"])
        .assert()
        .success();

    assert_eq!(get_current_branch(dir.path()), "develop");
    assert!(tag_exists(dir.path(), "v1.1.1"));
}

#[test]
fn test_bugfix_workflow() {
    let dir = tempdir().expect("Failed to create temp dir");
    setup_repo(dir.path());

    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["init", "-d"])
        .assert()
        .success();

    // Bugfix Start
    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["bugfix", "start", "fix-me"])
        .assert()
        .success();
    assert_eq!(get_current_branch(dir.path()), "bugfix/fix-me");

    // Bugfix Finish
    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["bugfix", "finish", "fix-me"])
        .assert()
        .success();

    assert_eq!(get_current_branch(dir.path()), "develop");
}

#[test]
fn test_support_workflow() {
    let dir = tempdir().expect("Failed to create temp dir");
    setup_repo(dir.path());

    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["init", "-d"])
        .assert()
        .success();

    // Support Start (requires base)
    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["support", "start", "old-version", "main"])
        .assert()
        .success();
    assert_eq!(get_current_branch(dir.path()), "support/old-version");
}

#[test]
fn test_init_no_git_repo() {
    let dir = tempdir().expect("Failed to create temp dir");
    // Don't setup_repo here

    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .arg("init")
        .arg("-d")
        .assert()
        .failure();
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("gitflow").unwrap();
    cmd.arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("Oxidized AVH Edition"));
}

#[test]
fn test_config_command() {
    let dir = tempdir().expect("Failed to create temp dir");
    setup_repo(dir.path());

    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["init", "-d"])
        .assert()
        .success();

    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["config", "list"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("gitflow.branch.master")
                .or(predicate::str::contains("gitflow.branch.main")),
        );
}

#[test]
fn test_feature_extended_actions() {
    let dir = tempdir().expect("Failed to create temp dir");
    setup_repo(dir.path());

    // Init
    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["init", "-d"])
        .assert()
        .success();

    // Feature Start
    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["feature", "start", "extended-feat"])
        .assert()
        .success();

    // Checkout develop
    Command::cargo_bin("gitflow")
        .unwrap()
        .current_dir(dir.path())
        .args(&["feature", "checkout", "extended-feat"])
        .assert()
        .success();
}
