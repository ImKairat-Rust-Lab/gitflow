use crate::cli::Cli;
use crate::error::AppError;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tracing::warn;

pub fn run_install() -> Result<(), AppError> {
    println!("📦 Starting gitflow installation...");
    
    // 1. Install binary
    install_binary()?;

    // 2. Setup completions
    let source_cmd = setup_completions()?;

    println!("\n🚀 Installation complete! Please restart your terminal or run:");
    if let Some(cmd) = source_cmd {
        println!("   {}", cmd);
    } else {
        println!("   # (restart your shell or source your specific config)");
    }
    Ok(())
}

fn install_binary() -> Result<(), AppError> {
    let home = env::var("HOME").map_err(|_| AppError::Config("HOME env var not set".into()))?;
    let target_dir = PathBuf::from(&home).join(".local").join("bin");

    if !target_dir.exists() {
        println!("📁 Creating directory: {}", target_dir.display());
        fs::create_dir_all(&target_dir).map_err(AppError::IoError)?;
    }

    let current_exe = env::current_exe().map_err(AppError::IoError)?;
    let target_path = target_dir.join("gitflow");

    println!("📄 Copying binary to: {}", target_path.display());
    fs::copy(&current_exe, &target_path).map_err(AppError::IoError)?;

    // Ensure it's executable (on Linux)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&target_path).map_err(AppError::IoError)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&target_path, perms).map_err(AppError::IoError)?;
        println!("🔧 Set executable permissions on binary");
    }

    println!("✅ Binary installed successfully");
    println!("💡 Make sure {} is in your PATH.", target_dir.display());
    
    Ok(())
}

fn setup_completions() -> Result<Option<String>, AppError> {
    let shell_path = env::var("SHELL").unwrap_or_default();
    let shell = if shell_path.contains("zsh") {
        Shell::Zsh
    } else if shell_path.contains("fish") {
        Shell::Fish
    } else if shell_path.contains("bash") {
        Shell::Bash
    } else {
        warn!("Unsupported shell for automatic completion setup: {}", shell_path);
        return Ok(None);
    };

    let home = env::var("HOME").unwrap_or_default();
    let mut cmd = Cli::command();
    let bin_name = "gitflow";
    
    match shell {
        Shell::Bash => {
            let bash_config = PathBuf::from(&home).join(".bashrc");
            let completion_dir = PathBuf::from(&home).join(".local").join("share").join("gitflow");
            if !completion_dir.exists() {
                println!("📁 Creating completions directory: {}", completion_dir.display());
                fs::create_dir_all(&completion_dir).map_err(AppError::IoError)?;
            }
            let completion_file = completion_dir.join("completion.bash");
            
            println!("📄 Generating Bash completions to: {}", completion_file.display());
            let mut file = fs::File::create(&completion_file).map_err(AppError::IoError)?;
            generate(Shell::Bash, &mut cmd, bin_name, &mut file);
            
            append_to_config(&bash_config, &format!("source {}", completion_file.display()))?;
            println!("✅ Bash completions installed and added to {}", bash_config.display());
            Ok(Some(format!("source {}", bash_config.display())))
        }
        Shell::Zsh => {
            let zsh_config = PathBuf::from(&home).join(".zshrc");
            let completion_dir = PathBuf::from(&home).join(".local").join("share").join("gitflow");
            if !completion_dir.exists() {
                println!("📁 Creating completions directory: {}", completion_dir.display());
                fs::create_dir_all(&completion_dir).map_err(AppError::IoError)?;
            }
            let completion_file = completion_dir.join("completion.zsh");
            
            println!("📄 Generating Zsh completions to: {}", completion_file.display());
            let mut file = fs::File::create(&completion_file).map_err(AppError::IoError)?;
            generate(Shell::Zsh, &mut cmd, bin_name, &mut file);
            
            append_to_config(&zsh_config, &format!("source {}", completion_file.display()))?;
            println!("✅ Zsh completions installed and added to {}", zsh_config.display());
            Ok(Some(format!("source {}", zsh_config.display())))
        }
        Shell::Fish => {
            let fish_config_dir = PathBuf::from(&home).join(".config").join("fish").join("completions");
            if !fish_config_dir.exists() {
                println!("📁 Creating Fish completions directory: {}", fish_config_dir.display());
                fs::create_dir_all(&fish_config_dir).map_err(AppError::IoError)?;
            }
            let completion_file = fish_config_dir.join("gitflow.fish");
            
            println!("📄 Generating Fish completions to: {}", completion_file.display());
            let mut file = fs::File::create(&completion_file).map_err(AppError::IoError)?;
            generate(Shell::Fish, &mut cmd, bin_name, &mut file);
            println!("✅ Fish completions installed");
            Ok(None)
        }
        _ => Ok(None),
    }
}

fn append_to_config(config_path: &PathBuf, line: &str) -> Result<(), AppError> {
    if !config_path.exists() {
        println!("⚠️  Config file {} not found, skipping.", config_path.display());
        return Ok(());
    }

    let content = fs::read_to_string(config_path).map_err(AppError::IoError)?;
    if content.contains(line) {
        println!("ℹ️  Completions already configured in {}", config_path.display());
        return Ok(());
    }

    println!("✍️  Appending completion config to {}", config_path.display());
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(config_path)
        .map_err(AppError::IoError)?;

    writeln!(file, "\n# gitflow auto-generated completion\n{}", line).map_err(AppError::IoError)?;
    Ok(())
}
