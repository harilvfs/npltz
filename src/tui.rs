use crate::app::{App, AppMode};
use crate::config::Config;
use crate::{log, ui};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io;
use std::time::Duration;

pub fn run_tui() -> io::Result<()> {
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
                AppMode::Normal => handle_normal_key(app, key.code),
                AppMode::ThemeSelector => handle_theme_selector_key(app, key.code),
                AppMode::Goto => handle_goto_key(app, key.code),
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
    if app.show_small_warning && key == KeyCode::Enter {
        app.show_small_warning = false;
    }

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
