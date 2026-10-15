use crate::app::App;
use crate::calendar;
use chrono::{Datelike, Local};

use super::{Mode, calendar_layout, day_labels, fit_spans};
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

pub fn render(frame: &mut Frame, area: Rect, app: &App, mode: Mode) {
    let layout = calendar_layout(area, mode, app.show_ad_rows);

    frame.render_widget(Clear, area);

    if layout.bordered {
        let block = Block::default()
            .title(" Nepali BS Calendar (Patro) ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(app.theme.primary));
        frame.render_widget(block, area);
    }

    if let Some(sidebar) = layout.sidebar {
        render_sidebar(frame, sidebar, app);
    }

    render_title(frame, layout.title, app);
    if let Some(status) = layout.status {
        frame.render_widget(Paragraph::new(status_line(app, status.width)), status);
    }
    render_header(frame, layout.header, app, layout.cell_w);
    render_grid(frame, layout.grid, app, &layout);

    if let Some(detail) = layout.detail {
        frame.render_widget(Paragraph::new(detail_line(app, detail.width)), detail);
    }
    if let Some(nav) = layout.nav {
        frame.render_widget(Paragraph::new(nav_line(app, nav.width)), nav);
    }
    if let Some(keys) = layout.keys {
        frame.render_widget(Paragraph::new(keys_line(app, keys.width)), keys);
    }
}

fn clip(text: &str, width: usize) -> String {
    if text.chars().count() <= width {
        text.to_string()
    } else {
        text.chars().take(width).collect()
    }
}

fn centered(text: String, width: usize, style: Style) -> Line<'static> {
    let text = clip(&text, width);
    let len = text.chars().count();
    let pad = width.saturating_sub(len) / 2;
    let right = width.saturating_sub(pad + len);
    Line::from(vec![
        Span::raw(" ".repeat(pad)),
        Span::styled(text, style),
        Span::raw(" ".repeat(right)),
    ])
}

fn render_sidebar(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.secondary));
    let inner = block.inner(area);

    let mut lines: Vec<Line> = Vec::new();
    if inner.width > 0 && inner.height > 0 {
        for section in sidebar_sections(app, inner.width) {
            if lines.len() + section.len() > inner.height as usize {
                break;
            }
            lines.extend(section);
        }
        frame.render_widget(Paragraph::new(lines), inner);
    }
    frame.render_widget(block, area);
}

fn sidebar_sections(app: &App, width: u16) -> Vec<Vec<Line<'static>>> {
    let width = width as usize;
    let now = Local::now();
    let theme = &app.theme;

    let label = Style::default().fg(theme.secondary);
    let value = Style::default().fg(theme.primary).add_modifier(Modifier::BOLD);
    let dim = Style::default().fg(theme.secondary).add_modifier(Modifier::DIM);
    let sep_style = Style::default().fg(theme.secondary);

    let center = |text: String, style: Style| centered(text, width, style);
    let sep = || Line::from(Span::styled("\u{2500}".repeat(width), sep_style));

    let mut sections: Vec<Vec<Line>> = Vec::new();
    sections.push(vec![center("Nepali Time".into(), theme.hl())]);

    let Some(ref nd) = app.today else {
        sections.push(vec![center("--".into(), dim)]);
        return sections;
    };

    sections.push(vec![center(format!("{} {}", nd.month_name(), nd.year), value), sep()]);

    let ad_date = calendar::bs_to_ad(nd.year, nd.month, nd.day);
    let mut head = vec![center(format!("{}, day {}", nd.day_name(), nd.day), label)];
    if let Some(ref d) = ad_date {
        head.push(center(d.format("%b %d, %Y").to_string(), dim));
    }
    head.push(sep());
    sections.push(head);

    sections.push(vec![center(now.format("%I:%M %p").to_string(), dim), sep()]);

    if let Some(total) = calendar::get_days_in_month(nd.year, nd.month) {
        let remaining = total.saturating_sub(nd.day);
        sections.push(vec![center(format!("{} days left", remaining), dim)]);

        if let (Some(day_of_year), Some(year_days)) = (
            calendar::get_day_of_year(nd.year, nd.month, nd.day),
            calendar::get_year_total_days(nd.year),
        ) {
            let progress = day_of_year as f64 / year_days as f64;
            let bar_width = width.saturating_sub(4).max(1);
            let filled = ((progress * bar_width as f64).round() as usize).min(bar_width);
            let empty = bar_width - filled;
            let bar = format!("{}{}", "\u{2588}".repeat(filled), "\u{2591}".repeat(empty));
            sections.push(vec![
                center(bar, Style::default().fg(theme.primary)),
                center(format!("{}% complete this year", (progress * 100.0) as u32), dim),
            ]);
        }
    }

    if let Some(ref d) = ad_date {
        let last_day = chrono::NaiveDate::from_ymd_opt(d.year(), 12, 28).unwrap_or(*d);
        sections.push(vec![center(
            format!("Week {}/{}", d.iso_week().week(), last_day.iso_week().week()),
            dim,
        )]);
    }

    sections
}

fn render_title(frame: &mut Frame, area: Rect, app: &App) {
    let title_style = Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD);
    let ad_style = Style::default().fg(app.theme.secondary).add_modifier(Modifier::ITALIC);

    let bs = format!(" {} {} ", calendar::english_month_name(app.view_month), app.view_year);
    let ad = format!(" {} ", app.ad_range_str);
    let total = area.width as usize;

    let line = if bs.chars().count() + ad.chars().count() <= total {
        let pad = total - bs.chars().count() - ad.chars().count();
        Line::from(vec![
            Span::styled(bs, title_style),
            Span::raw(" ".repeat(pad)),
            Span::styled(ad, ad_style),
        ])
    } else {
        Line::from(Span::styled(clip(&bs, total), title_style))
    };

    frame.render_widget(Paragraph::new(line), area);
}

fn render_header(frame: &mut Frame, area: Rect, app: &App, cell_w: u16) {
    let hdr = Style::default().fg(app.theme.secondary).add_modifier(Modifier::BOLD);
    let sat_hdr = Style::default().fg(app.theme.error).add_modifier(Modifier::BOLD);

    let spans: Vec<Span> = day_labels(cell_w)
        .iter()
        .enumerate()
        .map(|(i, &name)| {
            let s = format!("{:^w$}", name, w = cell_w as usize);
            if i == 6 { Span::styled(s, sat_hdr) } else { Span::styled(s, hdr) }
        })
        .collect();

    frame.render_widget(Paragraph::new(Line::from(spans)).alignment(Alignment::Center), area);
}

fn render_grid(frame: &mut Frame, area: Rect, app: &App, layout: &super::CalendarLayout) {
    let cell_w = layout.cell_w as usize;
    let focus = app.focus_day();

    let ad_style =
        Style::default().fg(app.theme.secondary).add_modifier(Modifier::DIM | Modifier::ITALIC);
    let sat_style = Style::default().fg(app.theme.error);
    let sun_style = Style::default().fg(app.theme.warning).add_modifier(Modifier::BOLD);
    let plain_style = Style::default().fg(app.theme.fg);

    let mut grid_lines: Vec<Line> = Vec::new();
    for row in &app.calendar_rows {
        let mut bs_spans = Vec::new();
        let mut ad_spans = Vec::new();

        for cell in &row.cells {
            let day_s = format!("{:^w$}", cell.day, w = cell_w);
            let ad_s = format!("{:^w$}", cell.ad_day, w = cell_w);

            if cell.day == 0 {
                let blank = " ".repeat(cell_w);
                bs_spans.push(Span::raw(blank.clone()));
                ad_spans.push(Span::raw(blank));
                continue;
            }

            let (bs_style, ad_cell_style) = if cell.is_today {
                (app.theme.hl(), app.theme.hl().add_modifier(Modifier::ITALIC))
            } else if cell.is_goto_target {
                (
                    Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD),
                    Style::default().fg(app.theme.primary).add_modifier(Modifier::ITALIC),
                )
            } else if focus == Some(cell.day) {
                (app.theme.hl_soft(), app.theme.hl_soft().add_modifier(Modifier::ITALIC))
            } else if cell.is_saturday {
                (sat_style, ad_style)
            } else if cell.is_sunday {
                (sun_style, ad_style)
            } else {
                (plain_style, ad_style)
            };

            bs_spans.push(Span::styled(day_s, bs_style));
            if layout.show_ad {
                ad_spans.push(Span::styled(ad_s, ad_cell_style));
            }
        }

        grid_lines.push(Line::from(bs_spans));
        if layout.show_ad {
            grid_lines.push(Line::from(ad_spans));
        }
    }

    frame.render_widget(Paragraph::new(grid_lines).alignment(Alignment::Center), area);
}

fn status_line(app: &App, width: u16) -> Line<'static> {
    let theme = &app.theme;
    let now = Local::now();
    let dim = Style::default().fg(theme.secondary).add_modifier(Modifier::DIM);

    let mut segs: Vec<(String, Style)> = Vec::new();
    if let Some(ref nd) = app.today {
        segs.push((
            format!("{} {}", nd.day_name(), nd.day),
            Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
        ));
        segs.push((
            format!("{:04}/{:02}/{:02}", nd.year, nd.month, nd.day),
            Style::default().fg(theme.fg),
        ));
        if let Some(ad) = calendar::bs_to_ad(nd.year, nd.month, nd.day) {
            segs.push((format!("AD {}", ad.format("%b %d, %Y")), dim));
        }
        if let Some(total) = calendar::get_days_in_month(nd.year, nd.month) {
            segs.push((format!("{}d left", total.saturating_sub(nd.day)), dim));
        }
        segs.push((now.format("%I:%M %p").to_string(), dim));
    } else {
        segs.push(("--".into(), dim));
    }

    Line::from(fit_spans(segs, " \u{b7} ", width))
}

fn detail_line(app: &App, width: u16) -> Line<'static> {
    let theme = &app.theme;

    if let Some(day) = app.focus_day() {
        let mut segs: Vec<(String, Style)> = vec![(
            format!("BS {:04}-{:02}-{:02}", app.view_year, app.view_month, day),
            Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
        )];

        if let Some(ad) = calendar::bs_to_ad(app.view_year, app.view_month, day) {
            if let Some(nepali) = calendar::ad_to_bs(ad.year(), ad.month(), ad.day()) {
                segs.push((nepali.day_name().to_string(), Style::default().fg(theme.fg)));
            }
            segs.push((
                format!("AD {}", ad.format("%Y-%m-%d")),
                Style::default().fg(theme.secondary),
            ));
            segs.push((ad.format("%A").to_string(), Style::default().fg(theme.secondary)));
        }

        return Line::from(fit_spans(segs, " \u{b7} ", width));
    }

    let key = Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD);
    let hint = Style::default().fg(theme.secondary).add_modifier(Modifier::DIM);
    let segs = vec![
        ("Tab: pick a day".to_string(), key),
        ("a: AD rows".to_string(), hint),
        ("?: help".to_string(), hint),
    ];
    Line::from(fit_spans(segs, " \u{b7} ", width))
}

fn nav_line(app: &App, width: u16) -> Line<'static> {
    let (prev_m, prev_y) = app.prev_month_info();
    let (next_m, next_y) = app.next_month_info();
    let prev_name = format!("\u{2190} {} {}", calendar::english_month_name(prev_m), prev_y);
    let next_name = format!("{} {} \u{2192}", calendar::english_month_name(next_m), next_y);

    let style = Style::default().fg(app.theme.secondary).add_modifier(Modifier::BOLD);
    let total = width as usize;

    if prev_name.chars().count() + next_name.chars().count() <= total {
        let pad = total - prev_name.chars().count() - next_name.chars().count();
        Line::from(vec![
            Span::styled(prev_name, style),
            Span::raw(" ".repeat(pad)),
            Span::styled(next_name, style),
        ])
    } else {
        let segs = vec![(prev_name, style), (next_name, Style::default().fg(app.theme.secondary))];
        Line::from(fit_spans(segs, " \u{b7} ", width))
    }
}

/// Shortcuts, most important first; the tail is dropped when narrow instead of wrapping.
fn keys_line(app: &App, width: u16) -> Line<'static> {
    let key = Style::default().fg(app.theme.secondary).add_modifier(Modifier::BOLD);
    let hints = [
        "h/l month",
        "Tab day",
        "? help",
        "q quit",
        "j/k year",
        "a AD rows",
        "t today",
        "g goto",
        "c theme",
        "y year view",
    ];
    let segs: Vec<(String, Style)> = hints.iter().map(|h| (h.to_string(), key)).collect();
    Line::from(fit_spans(segs, " \u{b7} ", width))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_app() -> App {
        App::new_test("catppuccin-mocha")
    }

    fn line_text(line: &Line) -> String {
        line.spans.iter().map(|s| s.content.to_string()).collect()
    }

    #[test]
    fn test_status_line_fits_narrow_screen() {
        let app = test_app();
        for width in [10u16, 20, 45, 80] {
            let text = line_text(&status_line(&app, width));
            assert!(text.chars().count() <= width as usize, "status '{}' wider than {width}", text);
        }
    }

    #[test]
    fn test_status_line_drops_low_priority_segments() {
        let mut app = test_app();
        app.today = crate::calendar::ad_to_bs(2026, 9, 29);
        let wide = line_text(&status_line(&app, 120));
        let narrow = line_text(&status_line(&app, 30));
        assert!(wide.contains("left"));
        assert!(narrow.len() <= 30);
        assert!(!narrow.contains("left"));
    }

    #[test]
    fn test_detail_line_fits() {
        let app = test_app();
        for width in [8u16, 20, 45, 100] {
            let text = line_text(&detail_line(&app, width));
            assert!(text.chars().count() <= width as usize, "'{text}' > {width}");
        }
    }

    #[test]
    fn test_detail_line_shows_both_calendars_when_selected() {
        let mut app = test_app();
        app.selected_day = Some(15);
        let text = line_text(&detail_line(&app, 120));
        assert!(text.contains("BS 2081-01-15"));
        assert!(text.contains("AD "));
    }

    #[test]
    fn test_detail_hint_without_selection() {
        let app = test_app();
        let text = line_text(&detail_line(&app, 120));
        assert!(text.contains("Tab"));
        assert!(text.contains("?"));
    }

    #[test]
    fn test_keys_line_fits() {
        let app = test_app();
        for width in [12u16, 30, 45, 100] {
            let text = line_text(&keys_line(&app, width));
            assert!(text.chars().count() <= width as usize, "'{text}' > {width}");
        }
    }

    #[test]
    fn test_keys_line_keeps_essentials_on_narrow() {
        let app = test_app();
        let text = line_text(&keys_line(&app, 45));
        assert!(text.contains("h/l"));
        assert!(text.contains("?"));
        assert!(text.contains("q quit"));
    }

    #[test]
    fn test_sidebar_sections_fit_width() {
        let mut app = test_app();
        app.today = crate::calendar::ad_to_bs(2026, 9, 29);
        for width in [8u16, 14, 26, 40] {
            for section in sidebar_sections(&app, width) {
                for line in section {
                    assert!(line.width() <= width as usize, "sidebar line wider than {width}");
                }
            }
        }
    }

    #[test]
    fn test_clip_never_exceeds_width() {
        assert_eq!(clip("hello", 10), "hello");
        assert_eq!(clip("hello", 3).chars().count(), 3);
    }
}
