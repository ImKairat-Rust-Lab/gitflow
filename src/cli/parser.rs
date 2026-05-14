use crate::{
    cli::{Execute, GitFlowCommands},
    error::AppError,
};

impl Execute for GitFlowCommands {
    fn execute(self) -> Result<(), AppError> {
        match self {
            Self::Init(args) => args.execute(),
            Self::Feature { action } => action.execute(),
            Self::Release { action } => action.execute(),
            Self::Hotfix { action } => action.execute(),
            Self::Support { action } => action.execute(),
            Self::Bugfix { action } => action.execute(),
            Self::Finish => crate::commands::run_finish_auto(),
            Self::Install => crate::commands::run_install(),
        }
    }
}
