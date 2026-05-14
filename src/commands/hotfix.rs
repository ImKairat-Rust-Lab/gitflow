use crate::cli::{Execute, HotfixAction};
use crate::error::AppError;
use crate::git;
use crate::config::GitFlowConfig;
use tracing::info;

impl Execute for HotfixAction {
    fn execute(self) -> Result<(), AppError> {
        let config = GitFlowConfig::load()?;

        match self {
            Self::Start { version, base } => {
                let base = base.unwrap_or_else(|| config.main_branch.clone());
                let branch_name = format!("{}{}", config.hotfix_prefix, version);
                
                info!("Starting hotfix: {}", version);
                git::run_git_silent(&["checkout", "-b", &branch_name, &base], false)?;
            }
            Self::Finish { version, common, tagging, push } => {
                let branch_name = format!("{}{}", config.hotfix_prefix, version);
                let tag_name = format!("{}{}", config.version_tag_prefix, version);

                if common.fetch {
                    git::run_git_silent(&["fetch", "origin"], false)?;
                }

                // Merge into main
                git::run_git_silent(&["checkout", &config.main_branch], false)?;
                git::run_git_silent(&["merge", "--no-ff", &branch_name], false)?;

                // Tag
                if !tagging.notag {
                    let mut tag_args = vec!["tag".to_string()];
                    if tagging.sign {
                        tag_args.push("-s".to_string());
                    }
                    
                    let msg = tagging.message.unwrap_or_else(|| format!("Hotfix {}", version));
                    tag_args.push("-m".to_string());
                    tag_args.push(msg);
                    tag_args.push(tag_name.clone());
                    
                    let tag_args_refs: Vec<&str> = tag_args.iter().map(|s| s.as_str()).collect();
                    git::run_git_silent(&tag_args_refs, false)?;
                }

                // Merge into develop
                git::run_git_silent(&["checkout", &config.develop_branch], false)?;
                git::run_git_silent(&["merge", "--no-ff", &branch_name], false)?;

                // Delete branch
                if !common.keep {
                    git::run_git_silent(&["branch", "-d", &branch_name], false)?;
                }

                if push {
                    git::run_git_silent(&["push", "origin", &config.main_branch], false)?;
                    git::run_git_silent(&["push", "origin", &config.develop_branch], false)?;
                    if !tagging.notag {
                        git::run_git_silent(&["push", "origin", &tag_name], false)?;
                    }
                }

                info!("Hotfix '{}' finished. Merged into '{}' and '{}'.", version, config.main_branch, config.develop_branch);
            }
        }
        Ok(())
    }
}
