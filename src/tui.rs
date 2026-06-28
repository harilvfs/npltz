use crate::app::{App, AppMode};
use crate::config::Config;
use crate::{log, ui};
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEvent, MouseEventKind,
};
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

        if event::poll(tick_rate)? {
            loop {
                match event::read()? {
                    Event::Key(key) => match app.mode {
                        AppMode::Normal => handle_normal_key(app, key.code),
                        AppMode::ThemeSelector => handle_theme_selector_key(app, key.code),
                        AppMode::Goto => handle_goto_key(app, key.code),
                        AppMode::Help => handle_help_key(app, key.code),
                        AppMode::YearOverview => handle_year_overview_key(app, key.code),
                    },
                    Event::Mouse(mouse) => handle_mouse(app, mouse),
                    _ => {}
                }
                if !event::poll(Duration::ZERO)? {
                    break;
                }
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
        KeyCode::Char('?') => app.open_help(),
        KeyCode::Char('y') => app.open_year_overview(),
        _ => {}
    }
}

fn handle_help_key(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => app.close_help(),
        KeyCode::Char('j') | KeyCode::Down | KeyCode::Right => {
            app.help_scroll = (app.help_scroll + 1).min(app.help_max_scroll);
        }
        KeyCode::Char('k') | KeyCode::Up | KeyCode::Left => {
            app.help_scroll = app.help_scroll.saturating_sub(1);
        }
        KeyCode::PageDown => {
            app.help_scroll = (app.help_scroll + 10).min(app.help_max_scroll);
        }
        KeyCode::PageUp => {
            app.help_scroll = app.help_scroll.saturating_sub(10);
        }
        KeyCode::Home => {
            app.help_scroll = 0;
        }
        KeyCode::End => {
            app.help_scroll = app.help_max_scroll;
        }
        _ => {}
    }
}

fn handle_year_overview_key(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('y') => app.close_year_overview(),
        KeyCode::Char('h') | KeyCode::Left => app.year_prev(),
        KeyCode::Char('l') | KeyCode::Right => app.year_next(),
        KeyCode::Char('t') => app.year_today(),
        _ => {}
    }
}

fn handle_theme_selector_key(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('c') => app.close_theme_selector(),
        KeyCode::Enter | KeyCode::Char('l') | KeyCode::Char(' ') => app.apply_selected_theme(),
        KeyCode::Up | KeyCode::Char('k') => app.theme_selector_prev(),
        KeyCode::Down | KeyCode::Char('j') => app.theme_selector_next(),
        KeyCode::Home => app.theme_selector_first(),
        KeyCode::End => app.theme_selector_last(),
        _ => {}
    }
}

fn handle_goto_key(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('g') => app.close_goto(),
        KeyCode::Enter => app.apply_goto(),
        KeyCode::Char(c) if c.is_ascii_digit() || c == '-' => app.goto_input.push(c),
        KeyCode::Backspace => {
            app.goto_input.pop();
        }
        _ => {}
    }
}

fn handle_mouse(app: &mut App, mouse: MouseEvent) {
    if app.mode != AppMode::Normal {
        return;
    }
    match mouse.kind {
        MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
            let (term_w, term_h) = terminal_size().unwrap_or((80, 24));
            let nav_row = term_h.saturating_sub(4);
            if mouse.row >= nav_row {
                if mouse.column < term_w / 2 {
                    app.navigate_prev();
                } else {
                    app.navigate_next();
                }
            }
        }
        MouseEventKind::ScrollUp => {
            app.navigate_prev();
        }
        MouseEventKind::ScrollDown => {
            app.navigate_next();
        }
        _ => {}
    }
}

fn terminal_size() -> Option<(u16, u16)> {
    crossterm::terminal::size().ok()
}
