mod calendar_popup;
mod goto_popup;
mod help_popup;
mod theme_selector;
mod warning;
mod year_overview;

use crate::app::{App, AppMode};
use ratatui::Frame;
use ratatui::layout::Rect;

pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    calendar_popup::render(frame, area, app);

    match app.mode {
        AppMode::ThemeSelector => {
            let sel_area = centered_rect(50, 40, area);
            theme_selector::render(frame, sel_area, app);
        }
        AppMode::Goto => {
            let goto_area = centered_rect(50, 30, area);
            goto_popup::render(frame, goto_area, app);
        }
        AppMode::Help => {
            let help_area = centered_rect(50, 65, area);
            let max_scroll = help_popup::render(frame, help_area, app);
            app.help_max_scroll = max_scroll;
        }
        AppMode::YearOverview => {
            let overview_area = centered_rect(95, 95, area);
            year_overview::render(frame, overview_area, app);
        }
        AppMode::Normal => {}
    }

    if area.width < 80 && app.show_small_warning {
        warning::render(frame, area);
    }
}

pub(crate) fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_w = r.width * percent_x / 100;
    let popup_h = r.height * percent_y / 100;
    let offset_x = (r.width - popup_w) / 2;
    let offset_y = (r.height - popup_h) / 2;
    Rect { x: r.x + offset_x, y: r.y + offset_y, width: popup_w, height: popup_h }
}

pub(crate) const DAY_HEADER: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
