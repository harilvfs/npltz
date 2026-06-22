mod app;
mod calendar;
mod config;
mod error;
mod log;
mod theme;
mod ui;

use app::App;
use calendar::NepaliDate;
use chrono::{Datelike, Local, NaiveDate};
use clap::builder::styling::{AnsiColor, Style};
use clap::{ArgAction, CommandFactory, Parser, Subcommand};
use config::Config;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use error::{NpltzError, Result};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io;
use std::time::Duration;

pub fn styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .header(Style::new().fg_color(Some(AnsiColor::Green.into())).bold())
        .usage(Style::new().fg_color(Some(AnsiColor::Green.into())).bold())
        .literal(Style::new().fg_color(Some(AnsiColor::Cyan.into())))
        .placeholder(Style::new().fg_color(Some(AnsiColor::Cyan.into())))
}

#[derive(Parser)]
#[command(name = "npltz", about = "Nepali calendar and date converter for the terminal", styles = styles())]
pub struct Cli {
    #[arg(
        short = 'v',
        visible_short_alias = 'V',
        long = "version",
        action = ArgAction::SetTrue,
        help = "Print version information"
    )]
    version: bool,

    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(
        long,
        global = true,
        help = "Set default theme (catppuccin-mocha, dracula, gruvbox, nord, rose-pine, or default to reset)"
    )]
    set_theme: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Show today's Nepali and English date")]
    Show {
        #[arg(long, help = "Show date for a specific AD date (YYYY-MM-DD)")]
        date: Option<String>,

        #[arg(long, help = "Convert a BS date to AD (YYYY-MM-DD)")]
        bs: Option<String>,

        #[arg(long, help = "Output as JSON")]
        json: bool,
    },
    #[command(about = "Convert an AD date (YYYY-MM-DD) to Bikram Sambat")]
    Convert { date: String },
    #[command(about = "Convert a Bikram Sambat date (YYYY-MM-DD) to AD")]
    ConvertBs { date: String },
    #[command(about = "Generate shell completions (bash, zsh, fish)")]
    Completions { shell: clap_complete::Shell },
}

pub fn run() -> Result<()> {
    log::Log::init();
    let cli = Cli::parse();

    if cli.version {
        println!("npltz v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if let Some(theme) = &cli.set_theme {
        let mut config = Config::load();
        if theme == "default" {
            config.theme = None;
            config.save()?;
            println!("Default theme restored.");
        } else if theme::is_valid_theme(theme) {
            config.theme = Some(theme.clone());
            config.save()?;
            println!("{} has been set as your default theme.", theme::display_name(theme));
        } else {
            let msg = format!(
                "Unknown theme '{}'. Valid themes: {}",
                theme,
                theme::THEME_NAMES.join(", ")
            );
            log::Log::error(&msg);
            return Err(NpltzError::InvalidTheme(msg));
        }
        return Ok(());
    }

    match cli.command {
        Some(Commands::Show { date, bs, json }) => {
            if let Some(bs_date) = bs {
                show_bs_date(&bs_date, json)?;
            } else if let Some(ad_date) = date {
                show_ad_date(&ad_date, json)?;
            } else {
                show_today(json)?;
            }
        }
        Some(Commands::Convert { date }) => {
            convert_ad_to_bs(&date)?;
        }
        Some(Commands::ConvertBs { date }) => {
            convert_bs_to_ad(&date)?;
        }
        Some(Commands::Completions { shell }) => {
            let mut cmd = Cli::command();
            clap_complete::generate(shell, &mut cmd, "npltz", &mut std::io::stdout());
        }
        None => {
            run_tui()?;
        }
    }
    Ok(())
}

fn show_today(json: bool) -> Result<()> {
    let now = Local::now();
    let nd = calendar::ad_to_bs(now.year(), now.month(), now.day());

    if json {
        let obj = serde_json::json!({
            "nepali": nd.as_ref().map(|n| n.format_long()),
            "nepali_short": nd.as_ref().map(|n| format!("{:04}/{:02}/{:02}", n.year, n.month, n.day)),
            "english": now.format("%A, %B %d, %Y").to_string(),
            "english_short": now.format("%Y/%m/%d").to_string(),
            "time": now.format("%I:%M:%S %p").to_string(),
            "bs_year": nd.as_ref().map(|n| n.year),
            "bs_month": nd.as_ref().map(|n| n.month),
            "bs_day": nd.as_ref().map(|n| n.day),
            "bs_month_name": nd.as_ref().map(|n| n.month_name()),
            "bs_day_name": nd.as_ref().map(|n| n.day_name()),
            "ad_year": now.year(),
            "ad_month": now.month(),
            "ad_day": now.day(),
        });
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        return Ok(());
    }

    if let Some(ref nd) = nd {
        println!(
            "Nepali  : {:04}/{:02}/{:02}  ({} {}, {} {})",
            nd.year,
            nd.month,
            nd.day,
            nd.day_name(),
            nd.day,
            nd.month_name(),
            nd.year
        );
    } else {
        println!("Nepali  : N/A");
    }

    println!(
        "English : {}  ({}, {} {}, {})",
        now.format("%Y/%m/%d"),
        now.format("%A"),
        now.format("%B"),
        now.day(),
        now.year()
    );
    println!("Time    : {}", now.format("%I:%M:%S %p"));
    Ok(())
}

fn show_ad_date(date_str: &str, json: bool) -> Result<()> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| NpltzError::InvalidDate("Invalid date format. Use YYYY-MM-DD".into()))?;
    let nd = calendar::ad_to_bs(date.year(), date.month(), date.day());

    if json {
        let obj = serde_json::json!({
            "nepali": nd.as_ref().map(|n| n.format_long()),
            "nepali_short": nd.as_ref().map(|n| format!("{:04}/{:02}/{:02}", n.year, n.month, n.day)),
            "bs_year": nd.as_ref().map(|n| n.year),
            "bs_month": nd.as_ref().map(|n| n.month),
            "bs_day": nd.as_ref().map(|n| n.day),
            "bs_month_name": nd.as_ref().map(|n| n.month_name()),
            "bs_day_name": nd.as_ref().map(|n| n.day_name()),
            "ad_year": date.year(),
            "ad_month": date.month(),
            "ad_day": date.day(),
        });
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        return Ok(());
    }

    if let Some(ref nd) = nd {
        println!(
            "Nepali  : {:04}/{:02}/{:02}  ({} {}, {} {})",
            nd.year,
            nd.month,
            nd.day,
            nd.day_name(),
            nd.day,
            nd.month_name(),
            nd.year
        );
    } else {
        println!("Nepali  : N/A");
    }

    println!(
        "English : {}  ({}, {} {}, {})",
        date.format("%Y/%m/%d"),
        date.format("%A"),
        date.format("%B"),
        date.day(),
        date.year()
    );
    Ok(())
}

fn show_bs_date(date_str: &str, json: bool) -> Result<()> {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return Err(NpltzError::InvalidDate("Invalid date format. Use YYYY-MM-DD".into()));
    }
    let year: i32 = parts[0].parse().map_err(|_| NpltzError::InvalidDate("Invalid year".into()))?;
    let month: u32 =
        parts[1].parse().map_err(|_| NpltzError::InvalidDate("Invalid month".into()))?;
    let day: u32 = parts[2].parse().map_err(|_| NpltzError::InvalidDate("Invalid day".into()))?;

    let ad = calendar::bs_to_ad(year, month, day);
    let weekday = ad.map_or(0, |d| d.format("%w").to_string().parse().unwrap_or(0));
    let nd = NepaliDate { year, month, day, weekday };

    if json {
        let obj = serde_json::json!({
            "nepali": nd.format_long(),
            "nepali_short": format!("{:04}/{:02}/{:02}", nd.year, nd.month, nd.day),
            "english": ad.map(|d| d.format("%A, %B %d, %Y").to_string()),
            "english_short": ad.map(|d| d.format("%Y/%m/%d").to_string()),
            "bs_year": year,
            "bs_month": month,
            "bs_day": day,
            "bs_month_name": nd.month_name(),
        });
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        return Ok(());
    }

    println!(
        "Nepali  : {:04}/{:02}/{:02}  ({} {}, {} {})",
        nd.year,
        nd.month,
        nd.day,
        nd.day_name(),
        nd.day,
        nd.month_name(),
        nd.year
    );
    if let Some(ad) = ad {
        println!(
            "English : {}  ({}, {} {}, {})",
            ad.format("%Y/%m/%d"),
            ad.format("%A"),
            ad.format("%B"),
            ad.day(),
            ad.year()
        );
    } else {
        println!("English : N/A");
    }
    Ok(())
}

fn convert_ad_to_bs(date_str: &str) -> Result<()> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| NpltzError::InvalidDate("Invalid date format. Use YYYY-MM-DD".into()))?;
    match calendar::ad_to_bs(date.year(), date.month(), date.day()) {
        Some(nd) => {
            println!("{}", nd.format_long());
        }
        None => {
            let bs_end = calendar::BS_EPOCH_YEAR + 124;
            println!(
                "Date out of range. Supports AD ~{}-{} (BS {}-{})",
                1918,
                2042,
                calendar::BS_EPOCH_YEAR,
                bs_end
            );
        }
    }
    Ok(())
}

fn convert_bs_to_ad(date_str: &str) -> Result<()> {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return Err(NpltzError::InvalidDate("Invalid date format. Use YYYY-MM-DD".into()));
    }
    let year: i32 = parts[0].parse().map_err(|_| NpltzError::InvalidDate("Invalid year".into()))?;
    let month: u32 =
        parts[1].parse().map_err(|_| NpltzError::InvalidDate("Invalid month".into()))?;
    let day: u32 = parts[2].parse().map_err(|_| NpltzError::InvalidDate("Invalid day".into()))?;

    match calendar::bs_to_ad(year, month, day) {
        Some(ad) => {
            println!("{}, {} {}, {}", ad.format("%A"), ad.format("%B"), ad.day(), ad.year());
        }
        None => {
            let bs_end = calendar::BS_EPOCH_YEAR + 124;
            println!(
                "Date out of range. Supports BS {}-{} (AD ~{}-{})",
                calendar::BS_EPOCH_YEAR,
                bs_end,
                1918,
                2042
            );
        }
    }
    Ok(())
}

fn run_tui() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let config = Config::load();
    let theme_name = config.theme.clone().unwrap_or_else(|| "catppuccin-mocha".into());
    let mut app = App::new(&theme_name);
    log::Log::info(&format!("TUI started (theme: {})", theme_name));

    let tick_rate = Duration::from_millis(200);
    let res = run_app(&mut terminal, &mut app, tick_rate);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(e) = res {
        log::Log::error(&format!("TUI error: {e}"));
        eprintln!("Error: {}", e);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    tick_rate: Duration,
) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::render(frame, app))?;

        if event::poll(tick_rate)?
            && let Event::Key(key) = event::read()?
        {
            match app.mode {
                app::AppMode::Normal => handle_normal_key(app, key.code),
                app::AppMode::ThemeSelector => handle_theme_selector_key(app, key.code),
                app::AppMode::Goto => handle_goto_key(app, key.code),
            }
        }

        app.update();

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn handle_normal_key(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        KeyCode::Right | KeyCode::Char('l') => app.navigate_next(),
        KeyCode::Left | KeyCode::Char('h') => app.navigate_prev(),
        KeyCode::Up | KeyCode::Char('k') => app.navigate_year_next(),
        KeyCode::Down | KeyCode::Char('j') => app.navigate_year_prev(),
        KeyCode::Char('t') => app.navigate_today(),
        KeyCode::Char('c') => app.open_theme_selector(),
        KeyCode::Char('g') => app.open_goto(),
        _ => {}
    }
}

fn handle_theme_selector_key(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('c') => app.close_theme_selector(),
        KeyCode::Enter => app.apply_selected_theme(),
        KeyCode::Up | KeyCode::Char('k') => app.theme_selector_prev(),
        KeyCode::Down | KeyCode::Char('j') => app.theme_selector_next(),
        _ => {}
    }
}

fn handle_goto_key(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc | KeyCode::Char('q') => app.close_goto(),
        KeyCode::Enter => app.apply_goto(),
        KeyCode::Char(c) if c.is_ascii_digit() || c == '-' => app.goto_input.push(c),
        KeyCode::Backspace => {
            app.goto_input.pop();
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_no_args() {
        let cli = Cli::try_parse_from(["npltz"]).unwrap();
        assert!(cli.command.is_none());
        assert_eq!(cli.set_theme, None);
    }

    #[test]
    fn test_cli_show() {
        let cli = Cli::try_parse_from(["npltz", "show"]).unwrap();
        match cli.command.unwrap() {
            Commands::Show { date, bs, json } => {
                assert!(date.is_none());
                assert!(bs.is_none());
                assert!(!json);
            }
            _ => panic!("Expected Show command"),
        }
    }

    #[test]
    fn test_cli_show_json() {
        let cli = Cli::try_parse_from(["npltz", "show", "--json"]).unwrap();
        match cli.command.unwrap() {
            Commands::Show { json, .. } => assert!(json),
            _ => panic!("Expected Show command"),
        }
    }

    #[test]
    fn test_cli_show_date() {
        let cli = Cli::try_parse_from(["npltz", "show", "--date", "2024-04-13"]).unwrap();
        match cli.command.unwrap() {
            Commands::Show { date, bs, .. } => {
                assert_eq!(date.unwrap(), "2024-04-13");
                assert!(bs.is_none());
            }
            _ => panic!("Expected Show command"),
        }
    }

    #[test]
    fn test_cli_show_bs() {
        let cli = Cli::try_parse_from(["npltz", "show", "--bs", "2081-01-01"]).unwrap();
        match cli.command.unwrap() {
            Commands::Show { bs, .. } => assert_eq!(bs.unwrap(), "2081-01-01"),
            _ => panic!("Expected Show command"),
        }
    }

    #[test]
    fn test_cli_convert() {
        let cli = Cli::try_parse_from(["npltz", "convert", "2024-04-13"]).unwrap();
        match cli.command.unwrap() {
            Commands::Convert { date } => assert_eq!(date, "2024-04-13"),
            _ => panic!("Expected Convert command"),
        }
    }

    #[test]
    fn test_cli_convert_bs() {
        let cli = Cli::try_parse_from(["npltz", "convert-bs", "2081-01-01"]).unwrap();
        match cli.command.unwrap() {
            Commands::ConvertBs { date } => assert_eq!(date, "2081-01-01"),
            _ => panic!("Expected ConvertBs command"),
        }
    }

    #[test]
    fn test_cli_set_theme() {
        let cli = Cli::try_parse_from(["npltz", "--set-theme", "dracula"]).unwrap();
        assert_eq!(cli.set_theme.unwrap(), "dracula");
    }

    #[test]
    fn test_cli_invalid_command_fails() {
        let result = Cli::try_parse_from(["npltz", "unknown"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_set_theme_invalid_still_parses() {
        let cli = Cli::try_parse_from(["npltz", "--set-theme", "invalid"]).unwrap();
        assert_eq!(cli.set_theme.unwrap(), "invalid");
    }
}
