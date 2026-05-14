use crate::error::AppError;
use std::process::Command;
use crate::git::repo::get_repo_root;

pub fn run_hook(name: &str, args: &[&str]) -> Result<(), AppError> {
    let root = get_repo_root()?;
    let hook_path = root.join(".git").join("hooks").join(name);

    if hook_path.exists() {
        tracing::info!("Running hook: {}", name);
        let status = Command::new(&hook_path)
            .args(args)
            .status()
            .map_err(AppError::IoError)?;

        if !status.success() {
            return Err(AppError::Config(format!(
                "Hook '{}' failed with exit code {}",
                name,
                status.code().unwrap_or(-1)
            )));
        }
    }
    Ok(())
}
