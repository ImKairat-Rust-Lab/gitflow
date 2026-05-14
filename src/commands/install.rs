use crate::cli::Cli;
use crate::error::AppError;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tracing::{info, warn};

pub fn run_install() -> Result<(), AppError> {
    // 1. Install binary
    install_binary()?;

    // 2. Setup completions
    setup_completions()?;

    info!("🚀 Installation complete! Please restart your terminal or source your config file.");
    Ok(())
}

fn install_binary() -> Result<(), AppError> {
    let home = env::var("HOME").map_err(|_| AppError::Config("HOME env var not set".into()))?;
    let target_dir = PathBuf::from(&home).join(".local").join("bin");

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).map_err(AppError::IoError)?;
    }

    let current_exe = env::current_exe().map_err(AppError::IoError)?;
    let target_path = target_dir.join("gitflow");

    info!("Installing binary to {}...", target_path.display());
    fs::copy(&current_exe, &target_path).map_err(AppError::IoError)?;

    // Ensure it's executable (on Linux)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&target_path).map_err(AppError::IoError)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&target_path, perms).map_err(AppError::IoError)?;
    }

    info!("✅ Binary installed to {}", target_path.display());
    info!("💡 Make sure {} is in your PATH.", target_dir.display());
    
    Ok(())
}

fn setup_completions() -> Result<(), AppError> {
    let shell_path = env::var("SHELL").unwrap_or_default();
    let shell = if shell_path.contains("zsh") {
        Shell::Zsh
    } else if shell_path.contains("fish") {
        Shell::Fish
    } else if shell_path.contains("bash") {
        Shell::Bash
    } else {
        warn!("Unsupported shell for automatic completion setup: {}", shell_path);
        return Ok(());
    };

    let home = env::var("HOME").unwrap_or_default();
    let mut cmd = Cli::command();
    let bin_name = "gitflow";
    
    match shell {
        Shell::Bash => {
            let bash_config = PathBuf::from(&home).join(".bashrc");
            let completion_dir = PathBuf::from(&home).join(".local").join("share").join("gitflow");
            fs::create_dir_all(&completion_dir).map_err(AppError::IoError)?;
            let completion_file = completion_dir.join("completion.bash");
            
            let mut file = fs::File::create(&completion_file).map_err(AppError::IoError)?;
            generate(Shell::Bash, &mut cmd, bin_name, &mut file);
            
            append_to_config(&bash_config, &format!("source {}", completion_file.display()))?;
            info!("✅ Bash completions installed and added to .bashrc");
        }
        Shell::Zsh => {
            let zsh_config = PathBuf::from(&home).join(".zshrc");
            let completion_dir = PathBuf::from(&home).join(".local").join("share").join("gitflow");
            fs::create_dir_all(&completion_dir).map_err(AppError::IoError)?;
            let completion_file = completion_dir.join("completion.zsh");
            
            let mut file = fs::File::create(&completion_file).map_err(AppError::IoError)?;
            generate(Shell::Zsh, &mut cmd, bin_name, &mut file);
            
            append_to_config(&zsh_config, &format!("source {}", completion_file.display()))?;
            info!("✅ Zsh completions installed and added to .zshrc");
        }
        Shell::Fish => {
            let fish_config_dir = PathBuf::from(&home).join(".config").join("fish").join("completions");
            fs::create_dir_all(&fish_config_dir).map_err(AppError::IoError)?;
            let completion_file = fish_config_dir.join("gitflow.fish");
            
            let mut file = fs::File::create(&completion_file).map_err(AppError::IoError)?;
            generate(Shell::Fish, &mut cmd, bin_name, &mut file);
            info!("✅ Fish completions installed to {}", completion_file.display());
        }
        _ => {}
    }

    Ok(())
}

fn append_to_config(config_path: &PathBuf, line: &str) -> Result<(), AppError> {
    if !config_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(config_path).map_err(AppError::IoError)?;
    if content.contains(line) {
        return Ok(());
    }

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(config_path)
        .map_err(AppError::IoError)?;

    writeln!(file, "\n# gitflow auto-generated completion\n{}", line).map_err(AppError::IoError)?;
    Ok(())
}
