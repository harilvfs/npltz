const HELP: &str = "\
Usage: cargo xtask <COMMAND>

Commands:
  ci    Run all CI checks
";

fn main() -> anyhow::Result<()> {
    let mut args = pico_args::Arguments::from_env();
    if args.contains(["-h", "--help"]) {
        print!("{HELP}");
        return Ok(());
    }

    let cmd = args.subcommand()?.unwrap_or_else(|| "ci".to_string());
    let sh = xshell::Shell::new()?;

    match cmd.as_str() {
        "ci" => ci(&sh),
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
