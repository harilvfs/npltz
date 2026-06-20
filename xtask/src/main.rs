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
    println!("→ cargo fmt --all --check");
    xshell::cmd!(sh, "cargo fmt --all --check").run()?;

    println!("→ cargo clippy --workspace -- -D warnings");
    xshell::cmd!(sh, "cargo clippy --workspace -- -D warnings").run()?;

    println!("→ cargo check --workspace");
    xshell::cmd!(sh, "cargo check --workspace").run()?;

    println!("→ cargo test --workspace");
    xshell::cmd!(sh, "cargo test --workspace").run()?;

    println!();
    println!("All CI checks passed.");
    Ok(())
}
