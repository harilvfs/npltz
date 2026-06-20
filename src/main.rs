mod app;
mod calendar;
mod theme;
mod ui;

use app::App;
use chrono::{Datelike, Local};
use clap::builder::styling::{AnsiColor, Style};
use clap::{Parser, Subcommand};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io;
use std::time::Duration;

fn styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .header(Style::new().fg_color(Some(AnsiColor::Green.into())).bold())
        .usage(Style::new().fg_color(Some(AnsiColor::Green.into())).bold())
        .literal(Style::new().fg_color(Some(AnsiColor::Cyan.into())))
        .placeholder(Style::new().fg_color(Some(AnsiColor::Cyan.into())))
}

#[derive(Parser)]
#[command(name = "npltz", about = "Nepali Patro in your terminal", styles = styles())]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Show today's Nepali and English date")]
    Show,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Show) => {
            show_today();
            Ok(())
        }
        None => run_tui(),
    }
}

fn show_today() {
    let now = Local::now();
    let nd = calendar::ad_to_bs(now.year(), now.month(), now.day());

    if let Some(ref nd) = nd {
        println!("Nepali  : {} {}, {} {}", nd.day_name(), nd.day, nd.month_name(), nd.year);
    } else {
        println!("Nepali  : N/A");
    }

    println!("English : {}, {} {}, {}", now.format("%A"), now.format("%B"), now.day(), now.year());
    println!("Time    : {}", now.format("%I:%M:%S %p"));
}

fn run_tui() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_secs(1);

    let res = run_app(&mut terminal, &mut app, tick_rate);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(e) = res {
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
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    app.should_quit = true;
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    app.navigate_next();
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    app.navigate_prev();
                }
                KeyCode::Char('t') => {
                    app.navigate_today();
                }
                _ => {}
            }
        }

        app.update();

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
