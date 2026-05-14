use crate::error::AppError;
use crate::git;

/// Holds all gitflow branch and prefix settings read from `git config`.
#[derive(Debug, Clone)]
pub struct GitFlowConfig {
    pub main_branch: String,
    pub develop_branch: String,
    pub feature_prefix: String,
    pub release_prefix: String,
    pub hotfix_prefix: String,
    pub bugfix_prefix: String,
    pub support_prefix: String,
    pub version_tag_prefix: String,
}

impl GitFlowConfig {
    /// Load the gitflow configuration from the local git config.
    /// Returns an error if the repository hasn't been initialized with `gitflow init`.
    pub fn load() -> Result<Self, AppError> {
        let get = |key: &str| -> Result<String, AppError> {
            git::git_config_get(key).ok_or_else(|| {
                AppError::Config(format!(
                    "Key '{}' not found. Run `gitflow init` first.",
                    key
                ))
            })
        };

        Ok(Self {
            main_branch: get("gitflow.branch.main")?,
            develop_branch: get("gitflow.branch.develop")?,
            feature_prefix: get("gitflow.prefix.feature")?,
            release_prefix: get("gitflow.prefix.release")?,
            hotfix_prefix: get("gitflow.prefix.hotfix")?,
            bugfix_prefix: get("gitflow.prefix.bugfix")?,
            support_prefix: get("gitflow.prefix.support")?,
            version_tag_prefix: get("gitflow.prefix.versiontag")?,
        })
    }
}
