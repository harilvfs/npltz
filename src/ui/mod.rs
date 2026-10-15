mod calendar_popup;
mod goto_popup;
mod help_popup;
mod theme_selector;
mod year_overview;

use crate::app::{App, AppMode};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Span;

/// Grid rows: two per week when AD dates are shown, one otherwise.
const GRID_ROWS_AD: u16 = 12;
const GRID_ROWS_PLAIN: u16 = 6;

const SIDEBAR_WIDTH: u16 = 30;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mode {
    Wide,
    Medium,
    Narrow,
}

pub(crate) fn layout_mode(area: Rect) -> Mode {
    if area.width >= 80 && area.height >= 24 {
        Mode::Wide
    } else if area.width >= 56 {
        Mode::Medium
    } else {
        Mode::Narrow
    }
}

/// Computed once per frame and shared with the mouse handler, so hit-testing
/// can never drift from rendering.
#[derive(Debug)]
pub(crate) struct CalendarLayout {
    pub bordered: bool,
    pub sidebar:  Option<Rect>,
    pub title:    Rect,
    pub status:   Option<Rect>,
    pub header:   Rect,
    pub grid:     Rect,
    pub detail:   Option<Rect>,
    pub nav:      Option<Rect>,
    pub keys:     Option<Rect>,
    pub show_ad:  bool,
    pub cell_w:   u16,
    /// X of the first day cell: the grid is drawn centred inside `grid`.
    pub grid_x:   u16,
}

pub(crate) fn calendar_layout(area: Rect, mode: Mode, want_ad: bool) -> CalendarLayout {
    let bordered = mode != Mode::Narrow && area.width >= 2 && area.height >= 2;
    let inner = if bordered {
        Rect {
            x:      area.x + 1,
            y:      area.y + 1,
            width:  area.width - 2,
            height: area.height - 2,
        }
    } else {
        area
    };

    let (sidebar, content) = if mode == Mode::Wide && inner.width >= 40 {
        let sb = SIDEBAR_WIDTH.min(inner.width / 3).max(1);
        (
            Some(Rect { x: inner.x, y: inner.y, width: sb, height: inner.height }),
            Rect {
                x:      inner.x + sb,
                y:      inner.y,
                width:  inner.width - sb,
                height: inner.height,
            },
        )
    } else {
        (None, inner)
    };

    let after_total: u16 = if mode == Mode::Narrow { 2 } else { 3 };
    let fixed: u16 = 1 + u16::from(sidebar.is_none()) + 1; // title, status, header
    let avail = content.height.saturating_sub(fixed);

    let show_ad = want_ad && avail >= GRID_ROWS_AD + after_total;
    let grid_rows = if show_ad { GRID_ROWS_AD } else { GRID_ROWS_PLAIN.min(avail) };
    let after = avail.saturating_sub(grid_rows).min(after_total);

    let has_detail = after >= 1;
    let has_keys = after >= 2;
    let has_nav = mode != Mode::Narrow && after >= 3;

    // Centre the grid; footers are pinned to the bottom below.
    let used = fixed + grid_rows + u16::from(has_detail) + u16::from(has_keys) + u16::from(has_nav);
    let pad_top = content.height.saturating_sub(used) / 2;

    let mut y = content.y + pad_top;
    let mut take = |h: u16| {
        let r = Rect { x: content.x, y, width: content.width, height: h };
        y += h;
        r
    };

    let title = take(1);
    let status = sidebar.is_none().then(|| take(1));
    let header = take(1);
    let grid = take(grid_rows);
    let detail = has_detail.then(|| take(1));

    let bottom = content.y + content.height;
    let keys = has_keys.then(|| Rect {
        x:      content.x,
        y:      bottom - 1,
        width:  content.width,
        height: 1,
    });
    let nav = has_nav.then(|| Rect {
        x:      content.x,
        y:      bottom - 2,
        width:  content.width,
        height: 1,
    });

    let cell_w = if grid.width >= 7 { (grid.width / 7).min(12) } else { 1 };
    let grid_width = cell_w * 7;
    let grid_x = grid.x + grid.width.saturating_sub(grid_width) / 2;

    CalendarLayout {
        bordered,
        sidebar,
        title,
        status,
        header,
        grid,
        detail,
        nav,
        keys,
        show_ad,
        cell_w,
        grid_x,
    }
}

pub(crate) enum Popup {
    Theme,
    Goto,
    Help,
    Year,
}

pub(crate) fn popup_rect(popup: Popup, mode: Mode, area: Rect) -> Rect {
    let (w, h) = match (popup, mode) {
        (Popup::Theme, Mode::Wide) => (50, 40),
        (Popup::Theme, Mode::Medium) => (70, 50),
        (Popup::Theme, _) => (90, 50),
        (Popup::Goto, Mode::Wide) => (50, 30),
        (Popup::Goto, Mode::Medium) => (70, 35),
        (Popup::Goto, _) => (90, 40),
        (Popup::Help, Mode::Wide) => (50, 65),
        (Popup::Help, Mode::Medium) => (80, 80),
        (Popup::Help, _) => (95, 90),
        (Popup::Year, _) => (95, 95),
    };
    centered_rect(w, h, area)
}

/// Join segments with `sep`, dropping the lowest-priority ones (from the end)
/// until the line fits `width`.
pub(crate) fn fit_spans(
    segments: Vec<(String, Style)>,
    sep: &str,
    width: u16,
) -> Vec<Span<'static>> {
    let width = width as usize;
    if width == 0 || segments.is_empty() {
        return Vec::new();
    }

    let len = |s: &str| s.chars().count();
    let mut kept = segments.len();
    loop {
        let text: usize = segments[..kept].iter().map(|(t, _)| len(t)).sum();
        let seps = sep.chars().count() * kept.saturating_sub(1);
        if text + seps <= width || kept == 1 {
            break;
        }
        kept -= 1;
    }

    let mut out: Vec<Span> = Vec::with_capacity(kept * 2);
    for (i, (text, style)) in segments[..kept].iter().enumerate() {
        if i > 0 {
            out.push(Span::raw(sep.to_string()));
        }
        if i == 0 && len(text) > width {
            // Clip the first segment and mark the cut with an ellipsis.
            let cut: String = text.chars().take(width.saturating_sub(1)).collect();
            out.push(Span::styled(format!("{cut}\u{2026}"), *style));
        } else {
            out.push(Span::styled(text.clone(), *style));
        }
    }
    out
}

pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let mode = layout_mode(area);

    calendar_popup::render(frame, area, app, mode);

    match app.mode {
        AppMode::ThemeSelector => {
            let sel_area = popup_rect(Popup::Theme, mode, area);
            theme_selector::render(frame, sel_area, app);
        }
        AppMode::Goto => {
            let goto_area = popup_rect(Popup::Goto, mode, area);
            goto_popup::render(frame, goto_area, app);
        }
        AppMode::Help => {
            let help_area = popup_rect(Popup::Help, mode, area);
            let max_scroll = help_popup::render(frame, help_area, app);
            app.help_max_scroll = max_scroll;
        }
        AppMode::YearOverview => {
            let overview_area = popup_rect(Popup::Year, mode, area);
            year_overview::render(frame, overview_area, app);
        }
        AppMode::Normal => {}
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

pub(crate) fn day_labels(cell_w: u16) -> [&'static str; 7] {
    if cell_w >= 5 {
        DAY_HEADER
    } else if cell_w >= 3 {
        ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"]
    } else {
        ["S", "M", "T", "W", "T", "F", "S"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(w: u16, h: u16) -> Rect {
        Rect::new(0, 0, w, h)
    }

    fn bottom(l: &CalendarLayout) -> u16 {
        let last = l.keys.or(l.nav).or(l.detail).unwrap_or(l.grid);
        last.y + last.height
    }

    #[test]
    fn test_mode_breakpoints() {
        assert_eq!(layout_mode(rect(100, 30)), Mode::Wide);
        assert_eq!(layout_mode(rect(80, 24)), Mode::Wide);
        assert_eq!(layout_mode(rect(100, 20)), Mode::Medium); // too short for a side panel
        assert_eq!(layout_mode(rect(70, 40)), Mode::Medium);
        assert_eq!(layout_mode(rect(45, 30)), Mode::Narrow);
        assert_eq!(layout_mode(rect(55, 30)), Mode::Narrow);
    }

    #[test]
    fn test_wide_layout_has_sidebar_and_ad_rows() {
        let l = calendar_layout(rect(100, 30), Mode::Wide, true);
        assert!(l.bordered);
        assert!(l.sidebar.is_some());
        assert!(l.show_ad);
        assert!(l.nav.is_some() && l.keys.is_some() && l.detail.is_some());
        assert_eq!(l.grid.height, 12);
        assert!(bottom(&l) <= 30);
    }

    #[test]
    fn test_narrow_layout_drops_chrome_when_short() {
        // A typical phone with the keyboard open: 45x16.
        let l = calendar_layout(rect(45, 16), Mode::Narrow, true);
        assert!(!l.bordered);
        assert!(l.sidebar.is_none());
        assert!(!l.show_ad);
        assert_eq!(l.grid.height, 6);
        assert!(l.nav.is_none());
        assert!(l.detail.is_some() && l.keys.is_some());
        assert!(bottom(&l) <= 16);
    }

    #[test]
    fn test_narrow_layout_shows_ad_rows_when_tall() {
        let l = calendar_layout(rect(45, 30), Mode::Narrow, true);
        assert!(l.show_ad);
        assert_eq!(l.grid.height, 12);
        assert!(bottom(&l) <= 30);
    }

    #[test]
    fn test_ad_rows_disabled_by_user() {
        let l = calendar_layout(rect(100, 40), Mode::Wide, false);
        assert!(!l.show_ad);
        assert_eq!(l.grid.height, 6);
    }

    #[test]
    fn test_layout_never_overflows() {
        for w in [10u16, 20, 40, 56, 79, 80, 120] {
            for h in [5u16, 8, 12, 16, 20, 24, 40] {
                let area = rect(w, h);
                let mode = layout_mode(area);
                let l = calendar_layout(area, mode, true);
                assert!(bottom(&l) <= h, "overflow at {w}x{h}: bottom {}", bottom(&l));
                assert!(l.grid.x + l.grid.width <= w.max(1));
            }
        }
    }

    #[test]
    fn test_cell_width_fits_grid() {
        let l = calendar_layout(rect(45, 30), Mode::Narrow, true);
        assert!(l.cell_w * 7 <= l.grid.width);
        assert!(l.cell_w >= 1);
    }

    #[test]
    fn test_fit_spans_drops_from_the_end() {
        let segs =
            |v: &[&str]| v.iter().map(|s| (s.to_string(), Style::default())).collect::<Vec<_>>();

        let all = fit_spans(segs(&["aa", "bb", "cc"]), " · ", 20);
        assert_eq!(all.len(), 5); // 3 segments + 2 separators

        let some = fit_spans(segs(&["aa", "bb", "cc"]), " · ", 7);
        assert_eq!(some.len(), 3); // "aa · bb"

        let one = fit_spans(segs(&["aa", "bb", "cc"]), " · ", 2);
        assert_eq!(one.len(), 1); // "aa"
    }

    #[test]
    fn test_fit_spans_truncates_oversized_first_segment() {
        let segs = vec![(("a".repeat(50)), Style::default())];
        let out = fit_spans(segs, " · ", 10);
        let text: String = out.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(text.chars().count(), 10);
        assert!(text.ends_with('\u{2026}'));
    }

    #[test]
    fn test_day_labels_shrink() {
        assert_eq!(day_labels(6), DAY_HEADER);
        assert_eq!(day_labels(3)[0], "Su");
        assert_eq!(day_labels(1)[0], "S");
    }

    #[test]
    fn test_render_every_size_without_panicking() {
        use crate::app::AppMode;
        use ratatui::Terminal;
        use ratatui::backend::TestBackend;

        let sizes = [
            (20u16, 8u16),
            (36, 12),
            (45, 16),
            (45, 30),
            (56, 20),
            (70, 24),
            (80, 24),
            (100, 30),
            (120, 40),
            (300, 90),
        ];

        for (w, h) in sizes {
            for plain in [false, true] {
                for mode in [AppMode::Normal, AppMode::Help, AppMode::YearOverview] {
                    let label = format!("{mode:?} plain={plain}");
                    let mut terminal = Terminal::new(TestBackend::new(w, h)).unwrap();
                    let mut app = App::new_test("catppuccin-mocha");
                    if plain {
                        app.theme = app.theme.into_plain(); // NO_COLOR rendering
                    }
                    app.mode = mode;
                    app.cursor_start();
                    terminal
                        .draw(|frame| render(frame, &mut app))
                        .unwrap_or_else(|e| panic!("draw failed at {w}x{h} ({label}): {e}"));
                }
            }
        }
    }

    /// Not a test: `cargo test dump_frames -- --ignored --nocapture` prints rendered frames.
    #[test]
    #[ignore]
    fn dump_frames() {
        use ratatui::Terminal;
        use ratatui::backend::TestBackend;

        for (w, h) in [(45u16, 16u16), (45, 30), (70, 24), (100, 30)] {
            let mut terminal = Terminal::new(TestBackend::new(w, h)).unwrap();
            let mut app = App::new_test("catppuccin-mocha");
            app.update();
            if let Some((y, m, _)) = app.today_key {
                app.view_year = y;
                app.view_month = m;
                app.build_view();
            }
            app.cursor_start();
            terminal.draw(|frame| render(frame, &mut app)).unwrap();

            let buf = terminal.backend().buffer();
            println!("+{}+", "-".repeat(w as usize));
            for y in 0..buf.area.height {
                let mut row = String::new();
                for x in 0..buf.area.width {
                    row.push_str(buf[(x, y)].symbol());
                }
                println!("|{row}|");
            }
            println!("+{}+\n", "-".repeat(w as usize));
        }
    }
}
