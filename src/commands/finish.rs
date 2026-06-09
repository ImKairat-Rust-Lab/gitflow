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

use crate::cli::Execute;
use crate::commands::common::{CommonFinishFlags, TaggingFlags};
use crate::commands::{BugfixAction, FeatureAction, HotfixAction, ReleaseAction};
use crate::config::GitFlowConfig;
use crate::error::AppError;
use crate::git;
use tracing::info;

pub fn run_finish_auto() -> Result<(), AppError> {
    let config = GitFlowConfig::load()?;
    let current = git::current_branch()?;

    let common = CommonFinishFlags {
        fetch: false,
        keep: false,
    };

    let tagging = TaggingFlags {
        sign: false,
        message: None,
        notag: false,
    };

    if current.starts_with(&config.feature_prefix) {
        let name = current
            .trim_start_matches(&config.feature_prefix)
            .to_string();
        info!("Auto-detected feature branch: {}", name);
        FeatureAction::Finish {
            name,
            common,
            rebase: false,
            squash: false,
            force_delete: false,
        }
        .execute()
    } else if current.starts_with(&config.release_prefix) {
        let version = current
            .trim_start_matches(&config.release_prefix)
            .to_string();
        info!("Auto-detected release branch: {}", version);
        ReleaseAction::Finish {
            version,
            common,
            tagging,
            push: false,
        }
        .execute()
    } else if current.starts_with(&config.hotfix_prefix) {
        let version = current
            .trim_start_matches(&config.hotfix_prefix)
            .to_string();
        info!("Auto-detected hotfix branch: {}", version);
        HotfixAction::Finish {
            version,
            common,
            tagging,
            push: false,
        }
        .execute()
    } else if current.starts_with(&config.bugfix_prefix) {
        let name = current
            .trim_start_matches(&config.bugfix_prefix)
            .to_string();
        info!("Auto-detected bugfix branch: {}", name);
        BugfixAction::Finish {
            name,
            common,
            rebase: false,
        }
        .execute()
    } else {
        Err(AppError::Config(format!(
            "Branch '{}' does not match any Gitflow prefixes. Please use specific finish commands (e.g., 'gitflow feature finish').",
            current
        )))
    }
}
