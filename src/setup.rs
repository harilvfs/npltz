use crate::cli::Cli;
use crate::error::Result;
use clap::CommandFactory;
use std::fs;
use std::path::PathBuf;

const DESKTOP_FILE: &str = include_str!("../npltz.desktop");

fn is_termux() -> bool {
    std::env::var("TERMUX_VERSION").is_ok() || std::env::var("PREFIX").is_ok()
}

fn is_root() -> bool {
    std::env::var("EUID").is_ok_and(|v| v == "0")
}

fn termux_prefix() -> PathBuf {
    std::env::var("PREFIX")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/data/data/com.termux/files/usr"))
}

fn bash_completion_dir() -> PathBuf {
    if is_termux() {
        termux_prefix().join("share/bash-completion/completions")
    } else if cfg!(target_os = "macos") {
        PathBuf::from("/usr/local/share/bash-completion/completions")
    } else if is_root() {
        PathBuf::from("/usr/share/bash-completion/completions")
    } else {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("~/.local/share"))
            .join("bash-completion/completions")
    }
}

fn zsh_completion_dir() -> PathBuf {
    if is_termux() {
        termux_prefix().join("share/zsh/site-functions")
    } else if cfg!(target_os = "macos") {
        PathBuf::from("/usr/local/share/zsh/site-functions")
    } else if is_root() {
        PathBuf::from("/usr/share/zsh/site-functions")
    } else {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("~/.local/share"))
            .join("zsh/site-functions")
    }
}

fn fish_completion_dir() -> PathBuf {
    if is_termux() {
        termux_prefix().join("share/fish/vendor_completions.d")
    } else if cfg!(target_os = "macos") {
        PathBuf::from("/usr/local/share/fish/vendor_completions.d")
    } else if is_root() {
        PathBuf::from("/usr/share/fish/vendor_completions.d")
    } else {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("~/.local/share"))
            .join("fish/vendor_completions.d")
    }
}

fn man_dir() -> PathBuf {
    if is_termux() {
        termux_prefix().join("share/man/man1")
    } else if cfg!(target_os = "macos") {
        PathBuf::from("/usr/local/share/man/man1")
    } else if is_root() {
        PathBuf::from("/usr/share/man/man1")
    } else {
        dirs::data_dir().unwrap_or_else(|| PathBuf::from("~/.local/share")).join("man/man1")
    }
}

fn write_file(path: &PathBuf, content: &[u8], dry_run: bool) -> Result<()> {
    if dry_run {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}

pub fn run_setup(dry_run: bool) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        println!("Setup is not required on Windows.");
        println!("Shell completions and man pages are not available for this platform.");
        return Ok(());
    }

    #[cfg(not(target_os = "windows"))]
    {
        let mut cmd = Cli::command();
        cmd = cmd.name("npltz");

        if dry_run {
            println!("(dry run) Would install:");
        }

        let mut installed = Vec::new();

        // Bash completions
        let bash_dir = bash_completion_dir();
        let bash_path = bash_dir.join("npltz");
        let mut bash_content = Vec::new();
        clap_complete::generate(
            clap_complete::Shell::Bash,
            &mut cmd.clone(),
            "npltz",
            &mut bash_content,
        );
        write_file(&bash_path, &bash_content, dry_run)?;
        installed.push(format!("bash completions → {}", bash_path.display()));

        // Zsh completions
        let zsh_dir = zsh_completion_dir();
        let zsh_path = zsh_dir.join("_npltz");
        let mut zsh_content = Vec::new();
        clap_complete::generate(
            clap_complete::Shell::Zsh,
            &mut cmd.clone(),
            "npltz",
            &mut zsh_content,
        );
        write_file(&zsh_path, &zsh_content, dry_run)?;
        installed.push(format!("zsh completions → {}", zsh_path.display()));

        // Fish completions
        let fish_dir = fish_completion_dir();
        let fish_path = fish_dir.join("npltz.fish");
        let mut fish_content = Vec::new();
        clap_complete::generate(
            clap_complete::Shell::Fish,
            &mut cmd.clone(),
            "npltz",
            &mut fish_content,
        );
        write_file(&fish_path, &fish_content, dry_run)?;
        installed.push(format!("fish completions → {}", fish_path.display()));

        // Man page
        let date = std::process::Command::new("date")
            .arg("+%B %d, %Y")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_else(|| "unknown".to_string());

        let man = clap_mangen::Man::new(cmd.clone())
            .title("npltz")
            .section("1")
            .date(date.trim())
            .source("npltz")
            .manual("npltz");

        let mut buf = Vec::new();
        man.render(&mut buf)?;

        let mut content = String::from_utf8(buf)
            .map_err(|e| crate::error::NpltzError::Config(format!("man page UTF-8 error: {e}")))?;
        for sub in cmd.get_subcommands() {
            let name = sub.get_name();
            let escaped = name.replace('-', "\\-");
            let old = format!("npltz\\-{escaped}(1)");
            let new = format!("\\fB{name}\\fR");
            content = content.replace(&old, &new);
        }
        content = content.replace("npltz\\-help(1)", "\\fBhelp\\fR");

        let man_dir = man_dir();
        let man_path = man_dir.join("npltz.1");
        write_file(&man_path, content.as_bytes(), dry_run)?;
        installed.push(format!("man page → {}", man_path.display()));

        // Desktop file (Linux only, not Termux)
        #[cfg(target_os = "linux")]
        {
            if !is_termux() {
                let desktop_dir = if is_root() {
                    PathBuf::from("/usr/share/applications")
                } else {
                    dirs::data_dir()
                        .unwrap_or_else(|| PathBuf::from("~/.local/share"))
                        .join("applications")
                };
                let desktop_path = desktop_dir.join("npltz.desktop");
                write_file(&desktop_path, DESKTOP_FILE.as_bytes(), dry_run)?;
                installed.push(format!("desktop file → {}", desktop_path.display()));
            }
        }

        for msg in &installed {
            if dry_run {
                println!("  {msg}");
            } else {
                println!("✓ {msg}");
            }
        }
    }

    Ok(())
}
