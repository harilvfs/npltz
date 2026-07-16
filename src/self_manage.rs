use crate::error::{NpltzError, Result};
use serde::Deserialize;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[cfg(not(target_os = "windows"))]
use std::path::Path;

const REPO: &str = "harilvfs/npltz";

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

enum InstallMethod {
    Cargo,
    CargoBinstall,
    InstallScript,
}

fn client() -> Result<reqwest::blocking::Client> {
    reqwest::blocking::Client::builder()
        .user_agent("npltz")
        .build()
        .map_err(|e| NpltzError::Config(format!("Failed to create HTTP client: {e}")))
}

fn command_exists(command: &str) -> bool {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {command}"))
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

fn get_latest_version() -> Result<String> {
    let resp = client()?
        .get(format!("https://api.github.com/repos/{REPO}/releases/latest"))
        .send()
        .map_err(|e| NpltzError::Config(format!("Failed to check for updates: {e}")))?;

    if !resp.status().is_success() {
        return Err(NpltzError::Config("Failed to fetch latest version from GitHub".into()));
    }

    let release: Release = resp
        .json()
        .map_err(|e| NpltzError::Config(format!("Failed to parse GitHub response: {e}")))?;

    Ok(release.tag_name.trim_start_matches('v').to_string())
}

#[cfg(not(target_os = "windows"))]
fn is_termux() -> bool {
    std::env::var("TERMUX_VERSION").is_ok() || std::env::var("PREFIX").is_ok()
}

#[cfg(not(target_os = "windows"))]
fn termux_prefix() -> PathBuf {
    std::env::var("PREFIX")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/data/data/com.termux/files/usr"))
}

#[cfg(not(target_os = "windows"))]
fn is_root() -> bool {
    std::env::var("EUID").is_ok_and(|v| v == "0")
}

fn get_install_method() -> Result<InstallMethod> {
    print!("Installed via (c)argo, (b)install, or (i)nstall script? ");
    io::stdout().flush()?;
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    match choice.trim().to_lowercase().as_str() {
        "c" | "cargo" => Ok(InstallMethod::Cargo),
        "b" | "binstall" | "cargo-binstall" => Ok(InstallMethod::CargoBinstall),
        "i" | "install script" => Ok(InstallMethod::InstallScript),
        _ => Err(NpltzError::Config("Invalid choice. Please run the command again.".into())),
    }
}

fn get_install_method_for_uninstall() -> Result<InstallMethod> {
    print!("Installed via (c)argo, (b)install, or (i)nstall script? ");
    io::stdout().flush()?;
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    match choice.trim().to_lowercase().as_str() {
        "c" | "cargo" => Ok(InstallMethod::Cargo),
        "b" | "binstall" | "cargo-binstall" => Ok(InstallMethod::CargoBinstall),
        "i" | "install script" => Ok(InstallMethod::InstallScript),
        _ => Err(NpltzError::Config("Invalid choice. Please run the command again.".into())),
    }
}

fn run_command(cmd: &mut std::process::Command) -> Result<()> {
    let status = cmd.status()?;
    if !status.success() {
        return Err(NpltzError::Config(format!("Command failed with exit code {status}")));
    }
    Ok(())
}

pub fn check_update() -> Result<()> {
    if !command_exists("npltz") {
        println!("npltz is not installed. Please install it first.");
        return Ok(());
    }

    println!("Checking for updates...");

    let current_version = env!("CARGO_PKG_VERSION");
    let latest_version = get_latest_version()?;

    println!("Current version: {current_version}");
    println!("Latest version:  {latest_version}");

    if latest_version == current_version {
        println!("\nYou are using the latest version of npltz.");
    } else {
        println!("\nUpdate available!");
        println!("Run 'npltz update' to update to the latest version.");
    }

    Ok(())
}

pub fn update() -> Result<()> {
    if !command_exists("npltz") {
        println!("npltz is not installed. Please install it first.");
        return Ok(());
    }

    println!("Updating npltz...");

    let method = get_install_method()?;

    match method {
        InstallMethod::Cargo => {
            run_command(
                std::process::Command::new("cargo").arg("install").arg("npltz").arg("--force"),
            )?;
        }
        InstallMethod::CargoBinstall => {
            run_command(
                std::process::Command::new("cargo").arg("binstall").arg("npltz").arg("--force"),
            )?;
        }
        InstallMethod::InstallScript => {
            #[cfg(target_os = "windows")]
            {
                println!("Automatic update is not available for Windows.");
                println!("Please download the latest binary from:");
                println!("  https://github.com/{REPO}/releases/latest");
                println!("Or update with: cargo install npltz --force");
            }

            #[cfg(not(target_os = "windows"))]
            {
                let script_url =
                    format!("https://raw.githubusercontent.com/{REPO}/main/install.sh");

                let resp = client()?.get(&script_url).send().map_err(|e| {
                    NpltzError::Config(format!("Failed to download install script: {e}"))
                })?;

                if !resp.status().is_success() {
                    return Err(NpltzError::Config(format!(
                        "Failed to download install script: HTTP {}",
                        resp.status()
                    )));
                }

                let script = resp.text().map_err(|e| {
                    NpltzError::Config(format!("Failed to read install script: {e}"))
                })?;

                let mut child = std::process::Command::new("sh")
                    .stdin(std::process::Stdio::piped())
                    .spawn()
                    .map_err(|e| {
                        NpltzError::Config(format!("Failed to run install script: {e}"))
                    })?;

                child
                    .stdin
                    .take()
                    .ok_or_else(|| NpltzError::Config("Failed to pipe install script".into()))?
                    .write_all(script.as_bytes())
                    .map_err(|e| {
                        NpltzError::Config(format!("Failed to pipe install script: {e}"))
                    })?;

                let status = child
                    .wait()
                    .map_err(|e| NpltzError::Config(format!("Install script failed: {e}")))?;

                if !status.success() {
                    return Err(NpltzError::Config(
                        "Install script failed. Try manually: curl -fsSL npltz.chalisehari.com.np/install | sh".into(),
                    ));
                }
            }
        }
    }

    Ok(())
}

pub fn uninstall() -> Result<()> {
    if !command_exists("npltz") {
        println!("npltz is not installed.");
        return Ok(());
    }

    println!("Uninstalling npltz...");

    let method = get_install_method_for_uninstall()?;

    match method {
        InstallMethod::Cargo | InstallMethod::CargoBinstall => {
            run_command(std::process::Command::new("cargo").arg("uninstall").arg("npltz"))?;
        }
        InstallMethod::InstallScript => {
            #[cfg(target_os = "windows")]
            {
                let config_dir = dirs::config_dir()
                    .unwrap_or_else(|| {
                        PathBuf::from(std::env::var("USERPROFILE").unwrap_or_default())
                    })
                    .join("npltz");

                if config_dir.exists() {
                    fs::remove_dir_all(&config_dir).ok();
                    println!("  Removed {}", config_dir.display());
                }

                if let Ok(exe_path) = std::env::current_exe() {
                    println!(
                        "\nPlease delete the binary manually after closing:\n  {}",
                        exe_path.display()
                    );
                }
            }

            #[cfg(not(target_os = "windows"))]
            {
                let home = std::env::var("HOME").ok();
                let is_termux = is_termux();
                let is_root = is_root();
                let is_macos = cfg!(target_os = "macos");

                let mut files: Vec<PathBuf> = Vec::new();

                if is_termux {
                    let prefix = termux_prefix();
                    files.extend(vec![
                        prefix.join("bin/npltz"),
                        prefix.join("share/bash-completion/completions/npltz"),
                        prefix.join("share/zsh/site-functions/_npltz"),
                        prefix.join("share/fish/vendor_completions.d/npltz.fish"),
                        prefix.join("share/man/man1/npltz.1"),
                    ]);
                } else if is_macos {
                    files.extend(vec![
                        PathBuf::from("/usr/local/bin/npltz"),
                        PathBuf::from("/usr/local/share/bash-completion/completions/npltz"),
                        PathBuf::from("/usr/local/share/zsh/site-functions/_npltz"),
                        PathBuf::from("/usr/local/share/fish/vendor_completions.d/npltz.fish"),
                        PathBuf::from("/usr/local/share/man/man1/npltz.1"),
                    ]);
                } else if is_root {
                    files.extend(vec![
                        PathBuf::from("/usr/local/bin/npltz"),
                        PathBuf::from("/usr/share/bash-completion/completions/npltz"),
                        PathBuf::from("/usr/share/zsh/site-functions/_npltz"),
                        PathBuf::from("/usr/share/fish/vendor_completions.d/npltz.fish"),
                        PathBuf::from("/usr/share/man/man1/npltz.1"),
                        PathBuf::from("/usr/share/applications/npltz.desktop"),
                    ]);
                } else if let Some(ref h) = home {
                    let data = PathBuf::from(h).join(".local/share");
                    files.extend(vec![
                        PathBuf::from("/usr/local/bin/npltz"),
                        data.join("bash-completion/completions/npltz"),
                        data.join("zsh/site-functions/_npltz"),
                        data.join("fish/vendor_completions.d/npltz.fish"),
                        data.join("man/man1/npltz.1"),
                        data.join("applications/npltz.desktop"),
                    ]);
                }

                let use_sudo = !is_termux && !is_root && !is_macos;

                for path in &files {
                    if path.exists() {
                        if use_sudo {
                            run_command(
                                std::process::Command::new("sudo").arg("rm").arg("-f").arg(path),
                            )?;
                        } else {
                            fs::remove_file(path).ok();
                        }
                        println!("  Removed {}", path.display());
                    }
                }

                if let Some(ref h) = home {
                    let config_dir = Path::new(h).join(".config/npltz");
                    if config_dir.exists() {
                        fs::remove_dir_all(&config_dir).ok();
                        println!("  Removed {}", config_dir.display());
                    }
                }
            }
        }
    }

    println!("\nnpltz has been removed.");
    Ok(())
}
