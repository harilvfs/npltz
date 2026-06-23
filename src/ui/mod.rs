mod calendar_popup;
mod goto_popup;
mod status_bar;
mod theme_selector;
mod warning;

use crate::app::{App, AppMode};
use ratatui::Frame;
use ratatui::layout::Rect;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let pct = if area.width < 60 { 95 } else { 80 };
    let popup_area = centered_rect(pct, 80, area);
    calendar_popup::render(frame, popup_area, app);

    let status_area = Rect {
        x:      area.x,
        y:      area.y + area.height.saturating_sub(1),
        width:  area.width,
        height: 1,
    };
    status_bar::render(frame, status_area, app);

    match app.mode {
        AppMode::ThemeSelector => {
            let sel_area = centered_rect(40, 30, area);
            theme_selector::render(frame, sel_area, app);
        }
        AppMode::Goto => {
            let goto_area = centered_rect(40, 20, area);
            goto_popup::render(frame, goto_area, app);
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
