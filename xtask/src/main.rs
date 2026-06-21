const HELP: &str = "\
Usage: cargo xtask <COMMAND>

Commands:
  ci         Run all CI checks
  man-pages  Generate manpage from clap definitions
";

fn main() -> anyhow::Result<()> {
    let mut args = pico_args::Arguments::from_env();
    if args.contains(["-h", "--help"]) {
        print!("{HELP}");
        return Ok(());
    }

    let sh = xshell::Shell::new()?;
    let cmd = args.subcommand()?.unwrap_or_else(|| "ci".to_string());

    match cmd.as_str() {
        "ci" => ci(&sh),
        "man-pages" | "man" => man_pages(&sh),
        _ => {
            eprintln!("Unknown command: {cmd}");
            print!("{HELP}");
            Err(anyhow::anyhow!("unknown command: {cmd}"))
        }
    }
}

fn ci(sh: &xshell::Shell) -> anyhow::Result<()> {
    xshell::cmd!(sh, "cargo +nightly fmt --all --check").run()?;
    xshell::cmd!(sh, "cargo +nightly clippy --workspace -- -D warnings").run()?;
    xshell::cmd!(sh, "cargo +nightly check --workspace").run()?;
    xshell::cmd!(sh, "cargo +nightly test --workspace").run()?;
    Ok(())
}

fn man_pages(sh: &xshell::Shell) -> anyhow::Result<()> {
    use clap::CommandFactory;

    let mut cmd = npltz::Cli::command();
    cmd = cmd.name("npltz");
    cmd = cmd.long_about(
        "A terminal-based Nepali calendar (Bikram Sambat) viewer with an interactive \
TUI and CLI conversion tools.\n\n\
npltz displays Nepali months with English dates alongside, \
supports AD to BS conversion, and offers simplicity.",
    );

    let date =
        String::from_utf8(std::process::Command::new("date").arg("+%B %d, %Y").output()?.stdout)?
            .trim()
            .to_string();

    let man = clap_mangen::Man::new(cmd.clone())
        .title("npltz")
        .section("1")
        .date(&date)
        .source("npltz")
        .manual("npltz");

    let out_dir = sh.current_dir();
    let man_path = out_dir.join("man/npltz.1");
    sh.create_dir("man")?;

    use std::io::Write;
    let mut buf = Vec::new();
    man.render(&mut buf)?;
    let mut content = String::from_utf8(buf)?;

    for sub in cmd.get_subcommands() {
        let name = sub.get_name();
        let escaped = name.replace('-', "\\-");
        let old = format!("npltz\\-{escaped}(1)");
        let new = format!("\\fB{name}\\fR");
        content = content.replace(&old, &new);
    }
    content = content.replace("npltz\\-help(1)", "\\fBhelp\\fR");

    let mut file = std::fs::File::create(&man_path)?;
    file.write_all(content.as_bytes())?;
    writeln!(
        file,
        r#".SH DOCUMENTATION
Documentation and source code available at:
.br
https://github.com/harilvfs/npltz

.SH AUTHOR
Aayush Chalise <me@aayush.xyz>

.SH REPORTING BUGS
If you encounter bugs or issues, please report them at:
.br
https://github.com/harilvfs/npltz/issues
"#
    )?;

    println!("Manpage generated at {}", man_path.display());
    Ok(())
}
