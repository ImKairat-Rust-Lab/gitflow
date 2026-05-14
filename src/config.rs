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
        let get_opt = |key: &str| git::git_config_get(key);
        
        let main_branch = get_opt("gitflow.branch.main")
            .or_else(|| get_opt("gitflow.branch.master"))
            .ok_or_else(|| AppError::Config("Could not find gitflow.branch.main or .master. Run `gitflow init` first.".into()))?;

        let develop_branch = get_opt("gitflow.branch.develop")
            .ok_or_else(|| AppError::Config("Key 'gitflow.branch.develop' not found.".into()))?;

        Ok(Self {
            main_branch,
            develop_branch,
            feature_prefix: get_opt("gitflow.prefix.feature").unwrap_or_else(|| "feature/".into()),
            release_prefix: get_opt("gitflow.prefix.release").unwrap_or_else(|| "release/".into()),
            hotfix_prefix: get_opt("gitflow.prefix.hotfix").unwrap_or_else(|| "hotfix/".into()),
            bugfix_prefix: get_opt("gitflow.prefix.bugfix").unwrap_or_else(|| "bugfix/".into()),
            support_prefix: get_opt("gitflow.prefix.support").unwrap_or_else(|| "support/".into()),
            version_tag_prefix: get_opt("gitflow.prefix.versiontag").unwrap_or_default(),
        })
    }
}
