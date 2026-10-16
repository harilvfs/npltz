#[cfg(target_os = "windows")]
pub fn run_setup(
    _dry_run: bool,
    _termux: bool,
    _boot: bool,
    _daily: bool,
) -> crate::error::Result<()> {
    println!("Setup is not required on Windows.");
    println!("Shell completions and man pages are not available for this platform.");
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn run_setup(dry_run: bool, termux: bool, boot: bool, daily: bool) -> crate::error::Result<()> {
    use crate::cli::Cli;
    use clap::CommandFactory;
    use std::fs;
    use std::path::PathBuf;

    #[cfg(target_os = "linux")]
    const DESKTOP_FILE: &str = "[Desktop Entry]
Name=npltz
Comment=Nepali BS Calendar (Patro) TUI for Terminal
Exec=npltz
Icon=utilities-terminal
Type=Application
Terminal=true
Categories=Utility;
Keywords=nepali;calendar;bikram-sambat;date;patro;tui;
";

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

    fn write_file(path: &PathBuf, content: &[u8], dry_run: bool) -> crate::error::Result<()> {
        if dry_run {
            return Ok(());
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
        Ok(())
    }

    let mut cmd = Cli::command();
    cmd = cmd.name("npltz");
    cmd = cmd.long_about(
        "A terminal-based Nepali calendar (Bikram Sambat) viewer with an interactive \
TUI and CLI conversion tools.\n\n\
npltz displays Nepali months with English dates alongside, \
supports AD to BS conversion, and offers simplicity.",
    );

    if dry_run {
        println!("(dry run) Would install:");
    }

    let mut installed = Vec::new();

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

    let zsh_dir = zsh_completion_dir();
    let zsh_path = zsh_dir.join("_npltz");
    let mut zsh_content = Vec::new();
    clap_complete::generate(clap_complete::Shell::Zsh, &mut cmd.clone(), "npltz", &mut zsh_content);
    write_file(&zsh_path, &zsh_content, dry_run)?;
    installed.push(format!("zsh completions → {}", zsh_path.display()));

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

    content.push_str(
        r#".SH DOCUMENTATION
Documentation and source code available at:
.br
https://github.com/harilvfs/npltz

.SH AUTHOR
Hari Chalise <harilvfs@chalisehari.com.np>

.SH REPORTING BUGS
If you encounter bugs or issues, please report them at:
.br
https://github.com/harilvfs/npltz/issues
"#,
    );

    let mdir = man_dir();
    let man_path = mdir.join("npltz.1");
    write_file(&man_path, content.as_bytes(), dry_run)?;
    installed.push(format!("man page → {}", man_path.display()));

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

    if termux {
        termux_extras(dry_run, boot, daily, &mut installed)?;
    }

    for msg in &installed {
        if dry_run {
            println!("  {msg}");
        } else {
            println!("✓ {msg}");
        }
    }

    Ok(())
}

/// Termux extras: session-menu shortcut, one-line task, extra-keys layout and
/// optional boot/daily notifications. Mirrors `contrib/termux/setup.sh`.
#[cfg(not(target_os = "windows"))]
fn termux_extras(
    dry_run: bool,
    boot: bool,
    daily: bool,
    installed: &mut Vec<String>,
) -> crate::error::Result<()> {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    const SHORTCUT: &str = concat!(
        "#!/data/data/com.termux/files/usr/bin/sh\n",
        "# Open the npltz TUI from the Termux session menu.\n",
        "exec npltz \"$@\"\n",
    );
    const TASK_TODAY: &str = concat!(
        "#!/data/data/com.termux/files/usr/bin/sh\n",
        "# Print today's BS/AD date as a single line.\n",
        "npltz show --oneline\n",
    );
    const NOTIFY_SCRIPT: &str = concat!(
        "#!/data/data/com.termux/files/usr/bin/sh\n",
        "# Post a notification with today's BS date.\n",
        "command -v termux-notification > /dev/null 2>&1 || exit 0\n",
        "command -v npltz > /dev/null 2>&1 || exit 0\n",
        "line=$(npltz show --oneline 2> /dev/null) || exit 0\n",
        "[ -n \"$line\" ] || exit 0\n",
        "termux-notification \\\n",
        "    --id npltz-today \\\n",
        "    --title \"Today in Bikram Sambat\" \\\n",
        "    --content \"$line\" \\\n",
        "    --priority low\n",
    );
    const EXTRA_KEYS: &str = "[\n  [\"Esc\", \"Tab\", \"h\", \"j\", \"k\", \"l\", \"q\"],\n  [\"CTRL\", \"left\", \"down\", \"up\", \"right\", \"a\", \"?\"]\n]\n";

    let Some(home) = dirs::home_dir() else {
        return Err(crate::error::NpltzError::Config("cannot determine home directory".into()));
    };

    if !dry_run && !(std::env::var("TERMUX_VERSION").is_ok() || std::env::var("PREFIX").is_ok()) {
        println!("! not a Termux environment, installing the extras anyway");
    }

    fn write_exec(
        path: &std::path::Path,
        content: &str,
        mode: u32,
        dry_run: bool,
    ) -> crate::error::Result<()> {
        if dry_run {
            return Ok(());
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
            // Termux ignores shortcuts in directories readable by others.
            let _ = fs::set_permissions(parent, fs::Permissions::from_mode(0o700));
        }
        fs::write(path, content)?;
        fs::set_permissions(path, fs::Permissions::from_mode(mode))?;
        Ok(())
    }

    let shortcuts = home.join(".shortcuts");
    let tasks = shortcuts.join("tasks");

    let shortcut_path = shortcuts.join("npltz");
    write_exec(&shortcut_path, SHORTCUT, 0o755, dry_run)?;
    installed.push(format!("termux shortcut → {}", shortcut_path.display()));

    let task_path = tasks.join("npltz-today");
    write_exec(&task_path, TASK_TODAY, 0o755, dry_run)?;
    installed.push(format!("termux task → {}", task_path.display()));

    // Never clobber a layout the user may have edited.
    let keys_path = home.join(".termux/extra-keys.json");
    if keys_path.exists() {
        installed.push(format!("termux extra keys → {} (kept existing)", keys_path.display()));
    } else {
        write_exec(&keys_path, EXTRA_KEYS, 0o644, dry_run)?;
        installed.push(format!("termux extra keys → {}", keys_path.display()));
    }

    if boot {
        let boot_path = home.join(".termux/boot/10-npltz-notify.sh");
        write_exec(&boot_path, NOTIFY_SCRIPT, 0o755, dry_run)?;
        installed.push(format!("termux boot notify → {}", boot_path.display()));
        if !dry_run {
            println!("  note: the boot script needs the termux-api package and app");
        }
    }

    if daily {
        let daily_path = tasks.join("npltz-notify-daily");
        write_exec(&daily_path, NOTIFY_SCRIPT, 0o755, dry_run)?;
        installed.push(format!("termux daily notify → {}", daily_path.display()));
        if dry_run {
            installed.push("termux job scheduler → would register daily job".to_string());
        } else {
            let cmd = std::process::Command::new("termux-job-scheduler")
                .args(["-s"])
                .arg(&daily_path)
                .args([
                    "--period-ms",
                    "86400000",
                    "--job-id",
                    "8310",
                    "--persisted",
                    "true",
                    "--network",
                    "none",
                ])
                .output();
            match cmd {
                Ok(o) if o.status.success() => {
                    installed.push("termux job scheduler → daily job registered".to_string());
                }
                Ok(o) => {
                    let err = String::from_utf8_lossy(&o.stderr);
                    let err = err.trim();
                    if err.is_empty() {
                        installed.push(
                            "termux job scheduler → failed to register (is termux-api installed?)"
                                .to_string(),
                        );
                    } else {
                        installed.push(format!(
                            "termux job scheduler → failed: {} — run manually: termux-job-scheduler \
                             -s {} --period-ms 86400000 --job-id 8310 --persisted true --network none",
                            err.lines().next().unwrap_or("unknown error"),
                            daily_path.display()
                        ));
                    }
                }
                Err(_) => installed.push(
                    "termux job scheduler → not found, run: termux-job-scheduler -s ... \
                     (see contrib/termux/README.md)"
                        .to_string(),
                ),
            }
        }
    }

    Ok(())
}
