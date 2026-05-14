use crate::cli::Execute;
use crate::error::AppError;
use crate::git;
use clap::Args;
use dialoguer::Input;
use tracing::info;

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Use default branch naming conventions
    #[arg(short = 'd', long)]
    pub default: bool,

    /// Force setting of gitflow branches, even if already configured
    #[arg(short = 'f', long)]
    pub force: bool,
}

impl Execute for InitArgs {
    /// Entry point for the `init` command.
    fn execute(self) -> Result<(), AppError> {
        // Helper: reads from git config, falls back to default if key is missing.
        let get_or_default = |key: &str, default: &str| -> String {
            // `git config --get` returns an error if the key doesn't exist.
            // We silently fall back to the default to avoid breaking the first run.
            git::run_git(&["config", "--get", key], false).unwrap_or_else(|_| default.to_string())
        };

        // Validator: rejects empty strings and whitespace.
        // Using `String` for errors avoids lifetime conflicts with `dialoguer`.
        let validator = |input: &String| -> Result<(), String> {
            if input.trim().is_empty() {
                Err("Value cannot be empty".into())
            } else if input.contains(char::is_whitespace) {
                Err("Value must not contain spaces".into())
            } else {
                Ok(())
            }
        };

        let (
            main_branch,
            develop_branch,
            feature_prefix,
            bugfix_prefix,
            release_prefix,
            hotfix_prefix,
            support_prefix,
            version_tag,
        ) = if self.default {
            (
                get_or_default("gitflow.branch.main", "main"),
                get_or_default("gitflow.branch.develop", "develop"),
                get_or_default("gitflow.prefix.feature", "feature/"),
                get_or_default("gitflow.prefix.bugfix", "bugfix/"),
                get_or_default("gitflow.prefix.release", "release/"),
                get_or_default("gitflow.prefix.hotfix", "hotfix/"),
                get_or_default("gitflow.prefix.support", "support/"),
                get_or_default("gitflow.prefix.versiontag", "v"),
            )
        } else {
            // 1. Sequential interactive prompts (matches original git-flow avh behavior)
            let main = prompt(
                "Which branch should be used for bringing forth production releases?",
                &get_or_default("gitflow.branch.main", "main"),
                validator,
            )?;
            let develop = prompt(
                "Which branch should be used for integration of the next release?",
                &get_or_default("gitflow.branch.develop", "develop"),
                validator,
            )?;
            let feature = prompt(
                "Branch prefix for features?",
                &get_or_default("gitflow.prefix.feature", "feature/"),
                validator,
            )?;
            let bugfix = prompt(
                "Branch prefix for bugfixes?",
                &get_or_default("gitflow.prefix.bugfix", "bugfix/"),
                validator,
            )?;
            let release = prompt(
                "Branch prefix for releases?",
                &get_or_default("gitflow.prefix.release", "release/"),
                validator,
            )?;
            let hotfix = prompt(
                "Branch prefix for hotfixes?",
                &get_or_default("gitflow.prefix.hotfix", "hotfix/"),
                validator,
            )?;
            let support = prompt(
                "Branch prefix for support branches?",
                &get_or_default("gitflow.prefix.support", "support/"),
                validator,
            )?;
            let vtag = prompt(
                "Version tag prefix?",
                &get_or_default("gitflow.prefix.versiontag", "v"),
                validator,
            )?;
            (
                main, develop, feature, bugfix, release, hotfix, support, vtag,
            )
        };

        // 2. Persist all settings to .git/config using `git config --local`
        let config_entries = [
            ("gitflow.branch.main", &main_branch),
            ("gitflow.branch.develop", &develop_branch),
            ("gitflow.prefix.feature", &feature_prefix),
            ("gitflow.prefix.bugfix", &bugfix_prefix),
            ("gitflow.prefix.release", &release_prefix),
            ("gitflow.prefix.hotfix", &hotfix_prefix),
            ("gitflow.prefix.support", &support_prefix),
            ("gitflow.prefix.versiontag", &version_tag),
        ];

        for (key, value) in config_entries {
            git::run_git_silent(&["config", "--local", key, value], false)?;
        }

        // 3. Create the develop branch if it doesn't exist yet
        let current = git::current_branch().unwrap_or_default();
        if current != develop_branch && !git::branch_exists(&develop_branch) {
            git::run_git_silent(&["checkout", "-b", &develop_branch, &main_branch], false)?;
        }

        info!(
            "✅ Git Flow initialized. Current branch: '{}'.",
            develop_branch
        );
        Ok(())
    }
}

/// Generic wrapper around `dialoguer::Input` for consistent prompt behavior.
fn prompt<F>(msg: &str, default: &str, validator: F) -> Result<String, AppError>
where
    F: Fn(&String) -> Result<(), String>,
{
    Input::new()
        .with_prompt(msg)
        .default(default.to_string())
        .show_default(true)
        .validate_with(validator)
        .interact()
        .map_err(|e| AppError::Config(format!("Input error: {}", e)))
}
