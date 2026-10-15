use crate::app::{App, AppMode};
use crate::config::Config;
use crate::ui::{self, Popup};
use crate::{log, theme};
use chrono::{Local, Timelike};
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEvent, MouseEventKind,
};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use std::io;
use std::time::Duration;

/// Idle poll interval. Nothing is redrawn unless something changed, so this only
/// decides how quickly a minute rollover is noticed.
const TICK: Duration = Duration::from_millis(500);

pub fn run_tui(no_color: bool) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let config = Config::load();
    let theme_name = config.theme.clone().unwrap_or_else(|| "catppuccin-mocha".into());
    let no_color = no_color || theme::no_color_requested();
    let mut app = App::new(&theme_name, no_color);
    log::Log::info(&format!("TUI started (theme: {})", theme_name));

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(e) = res {
        log::Log::error(&format!("TUI error: {e}"));
        eprintln!("Error: {}", e);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    let mut dirty = true;
    let mut last_minute = i32::MIN;

    loop {
        let minute = Local::now().minute() as i32;
        if dirty || minute != last_minute {
            terminal.draw(|frame| ui::render(frame, app))?;
            dirty = false;
            last_minute = minute;
        }

        if event::poll(TICK)? {
            loop {
                match event::read()? {
                    Event::Key(key) => {
                        match app.mode {
                            AppMode::Normal => handle_normal_key(app, key.code),
                            AppMode::ThemeSelector => handle_theme_selector_key(app, key.code),
                            AppMode::Goto => handle_goto_key(app, key.code),
                            AppMode::Help => handle_help_key(app, key.code),
                            AppMode::YearOverview => handle_year_overview_key(app, key.code),
                        }
                        dirty = true;
                    }
                    Event::Mouse(mouse) => {
                        handle_mouse(app, mouse);
                        dirty = true;
                    }
                    Event::Resize(_, _) => dirty = true,
                    _ => {}
                }
                if !event::poll(Duration::ZERO)? {
                    break;
                }
            }
        }

        if app.update() {
            dirty = true;
        }

        if app.should_quit {
            log::Log::info("TUI quit");
            break;
        }
    }
    Ok(())
}

fn handle_normal_key(app: &mut App, key: KeyCode) {
    if app.month_jump_pending {
        app.month_jump_pending = false;
        match key {
            KeyCode::Char('1') => app.jump_to_month(10),
            KeyCode::Char('2') => app.jump_to_month(11),
            KeyCode::Char('3') => app.jump_to_month(12),
            _ => {}
        }
        return;
    }

    // In cursor mode arrows move between days and Esc leaves the mode instead of quitting.
    if app.cursor {
        match key {
            KeyCode::Esc | KeyCode::Tab | KeyCode::BackTab => {
                app.cursor_stop();
                return;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                app.cursor_move(-1);
                return;
            }
            KeyCode::Right | KeyCode::Char('l') => {
                app.cursor_move(1);
                return;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.cursor_move(-7);
                return;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.cursor_move(7);
                return;
            }
            KeyCode::Char('t') => {
                app.cursor_today();
                return;
            }
            _ => {}
        }
    }

    match key {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Esc => {
            if app.cursor {
                app.cursor_stop();
            } else {
                app.should_quit = true;
            }
        }
        KeyCode::Tab | KeyCode::BackTab => app.cursor_start(),
        KeyCode::Right | KeyCode::Char('l') => app.navigate_next(),
        KeyCode::Left | KeyCode::Char('h') => app.navigate_prev(),
        KeyCode::Up | KeyCode::Char('k') => app.navigate_year_next(),
        KeyCode::Down | KeyCode::Char('j') => app.navigate_year_prev(),
        KeyCode::Char('t') => app.navigate_today(),
        KeyCode::Char('a') => app.toggle_ad_rows(),
        KeyCode::Char('c') => app.open_theme_selector(),
        KeyCode::Char('g') => app.open_goto(),
        KeyCode::Char('?') => app.open_help(),
        KeyCode::Char('y') => app.open_year_overview(),
        KeyCode::Char('0') => app.month_jump_pending = true,
        KeyCode::Char('1') => app.jump_to_month(1),
        KeyCode::Char('2') => app.jump_to_month(2),
        KeyCode::Char('3') => app.jump_to_month(3),
        KeyCode::Char('4') => app.jump_to_month(4),
        KeyCode::Char('5') => app.jump_to_month(5),
        KeyCode::Char('6') => app.jump_to_month(6),
        KeyCode::Char('7') => app.jump_to_month(7),
        KeyCode::Char('8') => app.jump_to_month(8),
        KeyCode::Char('9') => app.jump_to_month(9),
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
        KeyCode::Char(c) if c.is_ascii_digit() || c == '-' => {
            app.goto_error = None;
            app.goto_input.push(c);
        }
        KeyCode::Backspace => {
            app.goto_error = None;
            app.goto_input.pop();
        }
        _ => {}
    }
}

fn contains(r: Rect, x: u16, y: u16) -> bool {
    x >= r.x && x < r.x + r.width && y >= r.y && y < r.y + r.height
}

fn cell_at(layout: &ui::CalendarLayout, x: u16, y: u16) -> Option<(usize, usize)> {
    if layout.cell_w == 0 || !contains(layout.grid, x, y) {
        return None;
    }
    // The grid is centred in its rect, so measure from the first cell, not the edge.
    let rel_x = x.checked_sub(layout.grid_x)?;
    let col = (rel_x / layout.cell_w) as usize;
    if col >= 7 {
        return None;
    }
    let row_step = if layout.show_ad { 2 } else { 1 };
    let row = ((y - layout.grid.y) / row_step) as usize;
    Some((row, col))
}

fn handle_mouse(app: &mut App, mouse: MouseEvent) {
    let Some((w, h)) = terminal_size() else { return };
    let area = Rect::new(0, 0, w, h);
    let mode = ui::layout_mode(area);

    match app.mode {
        AppMode::Normal => {
            let layout = ui::calendar_layout(area, mode, app.show_ad_rows);
            match mouse.kind {
                MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
                    if let Some((row, col)) = cell_at(&layout, mouse.column, mouse.row) {
                        // Clicking a day pins it, like Tab.
                        if let Some(cell) =
                            app.calendar_rows.get(row).and_then(|r| r.cells.get(col))
                            && cell.day > 0
                        {
                            app.cursor = true;
                            app.selected_day = Some(cell.day);
                        }
                    } else if let Some(nav) = layout.nav
                        && contains(nav, mouse.column, mouse.row)
                    {
                        if mouse.column < nav.x + nav.width / 2 {
                            app.navigate_prev();
                        } else {
                            app.navigate_next();
                        }
                    }
                }
                MouseEventKind::ScrollUp => app.navigate_prev(),
                MouseEventKind::ScrollDown => app.navigate_next(),
                MouseEventKind::Moved => match cell_at(&layout, mouse.column, mouse.row) {
                    Some((row, col)) => app.update_hover(row, col),
                    None => app.clear_hover(),
                },
                _ => {}
            }
        }
        AppMode::Help => match mouse.kind {
            MouseEventKind::ScrollUp => {
                app.help_scroll = app.help_scroll.saturating_sub(3);
            }
            MouseEventKind::ScrollDown => {
                app.help_scroll = (app.help_scroll + 3).min(app.help_max_scroll);
            }
            _ => {}
        },
        AppMode::ThemeSelector => {
            let popup = ui::popup_rect(Popup::Theme, mode, area);
            match mouse.kind {
                MouseEventKind::ScrollUp => app.theme_selector_prev(),
                MouseEventKind::ScrollDown => app.theme_selector_next(),
                MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
                    // +2 = border + "Current:" line; -3 = border + 2-line footer.
                    let list_start = popup.y + 2;
                    let list_end = popup.y + popup.height.saturating_sub(3);
                    if mouse.row >= list_start && mouse.row < list_end {
                        let item_index = (mouse.row - list_start) as usize;
                        if item_index < theme::THEME_NAMES.len() {
                            app.theme_selector_selected = item_index;
                            app.apply_selected_theme();
                        }
                    }
                }
                _ => {}
            }
        }
        AppMode::YearOverview => {
            let popup = ui::popup_rect(Popup::Year, mode, area);
            match mouse.kind {
                MouseEventKind::ScrollUp => app.year_prev(),
                MouseEventKind::ScrollDown => app.year_next(),
                MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
                    let inner_x = popup.x + 1;
                    let inner_y = popup.y + 1;
                    let inner_w = popup.width.saturating_sub(2);
                    let inner_h = popup.height.saturating_sub(2);

                    let gap_x = 2u16;
                    let cols = 3u16;
                    let rows = 4u16;
                    let usable_w = inner_w.saturating_sub(gap_x * (cols - 1));
                    let col_w = usable_w / cols;
                    let row_h = inner_h / rows;

                    let rel_x = mouse.column.saturating_sub(inner_x);
                    let rel_y = mouse.row.saturating_sub(inner_y);

                    if rel_x < inner_w && rel_y < inner_h && col_w > 0 && row_h > 0 {
                        let col = (rel_x / (col_w + gap_x)).min(cols - 1);
                        let row = (rel_y / row_h).min(rows - 1);
                        let month = (row * cols + col + 1) as u32;
                        if (1..=12).contains(&month) {
                            app.view_month = month;
                            app.build_view();
                            app.close_year_overview();
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn terminal_size() -> Option<(u16, u16)> {
    crossterm::terminal::size().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Guards the desktop hover bug: a point in the last column must map to
    /// column 6 even though the grid is centred in a much wider rect.
    #[test]
    fn cell_at_accounts_for_centred_grid() {
        let area = Rect::new(0, 0, 120, 40);
        let layout = ui::calendar_layout(area, ui::Mode::Wide, false);

        let last_col_x = layout.grid_x + layout.cell_w * 6;
        let (row, col) = cell_at(&layout, last_col_x + 1, layout.grid.y).expect("hit");
        assert_eq!(col, 6, "far right of the grid must be Saturday");
        assert_eq!(row, 0);

        assert!(cell_at(&layout, layout.grid_x.saturating_sub(1), layout.grid.y).is_none());

        let past_grid = layout.grid_x + layout.cell_w * 7;
        if past_grid < layout.grid.x + layout.grid.width {
            assert!(cell_at(&layout, past_grid, layout.grid.y).is_none());
        }

        let ad_layout = ui::calendar_layout(area, ui::Mode::Wide, true);
        if ad_layout.show_ad {
            let (_, col) =
                cell_at(&ad_layout, ad_layout.grid_x, ad_layout.grid.y + 3).expect("hit");
            assert_eq!(col, 0);
        }
    }

    #[test]
    fn cell_at_on_narrow_screen() {
        let area = Rect::new(0, 0, 45, 16);
        let layout = ui::calendar_layout(area, ui::Mode::Narrow, false);
        let (row, col) = cell_at(&layout, layout.grid_x, layout.grid.y).expect("hit");
        assert_eq!((row, col), (0, 0));
    }
}
