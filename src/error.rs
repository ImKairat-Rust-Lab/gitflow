use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Git command failed: `git {args}` (exit code {status})\n{stderr}")]
    GitCommand {
        args: String,
        status: i32,
        stderr: String,
    },

    #[error("Config error: {0}")]
    Config(String),

    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),
}
