use crate::app::App;
use crate::calendar;
use chrono::Datelike;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

use super::DAY_HEADER;

const SIDEBAR_WIDTH: u16 = 30;

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Nepali BS Calendar (Patro) ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.primary))
        .title_bottom(
            Line::from(" h/l · j/k · c · g · y · t · ? · q ").alignment(Alignment::Center),
        );
    let inner = block.inner(area);

    let sb_w = SIDEBAR_WIDTH.min(inner.width / 3);
    let [sidebar_area, content_area] =
        Layout::horizontal([Constraint::Length(sb_w), Constraint::Min(0)]).areas(inner);

    render_sidebar(frame, sidebar_area, app);
    render_content(frame, content_area, app, inner);

    frame.render_widget(block, area);
}

fn render_sidebar(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.secondary));
    let inner = block.inner(area);

    let now = chrono::Local::now();
    let header_bg_style =
        Style::default().fg(app.theme.bg).bg(app.theme.primary).add_modifier(Modifier::BOLD);
    let label_style = Style::default().fg(app.theme.secondary);
    let value_style = Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD);
    let dim_style = Style::default().fg(app.theme.secondary).add_modifier(Modifier::DIM);
    let sep_style = Style::default().fg(app.theme.secondary);

    let w = (inner.width as usize).max(2);
    let sep = "─".repeat(w.saturating_sub(2));
    let mut lines: Vec<Line> = Vec::new();

    let header_text = "Nepali Time";
    let header_pad = w.saturating_sub(header_text.len()) / 2;
    let header_pad_r = w.saturating_sub(header_pad + header_text.len());
    let header_line =
        format!("{}{}{}", " ".repeat(header_pad), header_text, " ".repeat(header_pad_r));
    lines.push(Line::from(Span::styled(header_line, header_bg_style)));
    lines.push(Line::from(""));
    lines.push(Line::from(""));

    if let Some(ref nd) = app.today {
        let month = calendar::english_month_name(nd.month);
        let year_str = format!("{} Year", nd.year);
        let gap = w.saturating_sub(2 + month.len() + year_str.len());
        lines.push(Line::from(vec![
            Span::styled(" ", label_style),
            Span::styled(month, value_style),
            Span::styled(" ".repeat(gap), Style::default()),
            Span::styled(year_str, value_style),
        ]));
        lines.push(Line::from(""));

        lines.push(Line::from(Span::styled(format!(" {}", sep), sep_style)));
        lines.push(Line::from(""));

        let day_line_str = format!("{}, day {}", nd.day_name(), nd.day);
        let pad_day_line = w.saturating_sub(day_line_str.len()) / 2;
        lines.push(Line::from(vec![
            Span::styled(" ".repeat(pad_day_line), Style::default()),
            Span::styled(day_line_str, label_style),
        ]));

        let ad_date = calendar::bs_to_ad(nd.year, nd.month, nd.day);
        if let Some(ref date) = ad_date {
            let ad_str = date.format("%b %d, %Y").to_string();
            let pad_ad = w.saturating_sub(ad_str.len()) / 2;
            lines.push(Line::from(vec![
                Span::styled(" ".repeat(pad_ad), Style::default()),
                Span::styled(ad_str, dim_style),
            ]));
        }
        lines.push(Line::from(""));

        lines.push(Line::from(Span::styled(format!(" {}", sep), sep_style)));
        lines.push(Line::from(""));

        let time_str = now.format("%I:%M:%S %p").to_string();
        let pad_time = w.saturating_sub(time_str.len()) / 2;
        lines.push(Line::from(vec![
            Span::styled(" ".repeat(pad_time), Style::default()),
            Span::styled(time_str, dim_style),
        ]));
        lines.push(Line::from(""));

        lines.push(Line::from(Span::styled(format!(" {}", sep), sep_style)));
        lines.push(Line::from(""));

        if let Some(days_total) = calendar::get_days_in_month(nd.year, nd.month) {
            let remaining = days_total - nd.day;
            let remaining_str =
                format!("{} days left on {}", remaining, calendar::english_month_name(nd.month));
            let pad_remaining = w.saturating_sub(remaining_str.len()) / 2;
            lines.push(Line::from(vec![
                Span::styled(" ".repeat(pad_remaining), Style::default()),
                Span::styled(remaining_str, dim_style),
            ]));
            lines.push(Line::from(""));

            if let (Some(day_of_year), Some(year_days)) = (
                calendar::get_day_of_year(nd.year, nd.month, nd.day),
                calendar::get_year_total_days(nd.year),
            ) {
                let progress = day_of_year as f64 / year_days as f64;
                let bar_width = w.saturating_sub(4);
                let filled = (progress * bar_width as f64).round() as usize;
                let empty = bar_width.saturating_sub(filled);
                let bar_str = format!("{}{}", "\u{2588}".repeat(filled), "\u{2591}".repeat(empty));
                let pad_bar = (w.saturating_sub(bar_width)) / 2;
                lines.push(Line::from(vec![
                    Span::styled(" ".repeat(pad_bar), Style::default()),
                    Span::styled(bar_str, Style::default().fg(app.theme.primary)),
                ]));
                let pct_str = format!("{}% complete this year", (progress * 100.0) as u32);
                let pad_pct = w.saturating_sub(pct_str.len()) / 2;
                lines.push(Line::from(vec![
                    Span::styled(" ".repeat(pad_pct), Style::default()),
                    Span::styled(pct_str, dim_style),
                ]));
                lines.push(Line::from(""));
            }
        }

        if let Some(ref date) = ad_date {
            let week_num = date.iso_week().week();
            let last_day = chrono::NaiveDate::from_ymd_opt(date.year(), 12, 28).unwrap();
            let total_weeks = last_day.iso_week().week();
            let week_str = format!("Week {}/{}", week_num, total_weeks);
            let pad_week = w.saturating_sub(week_str.len()) / 2;
            lines.push(Line::from(vec![
                Span::styled(" ".repeat(pad_week), Style::default()),
                Span::styled(week_str, dim_style),
            ]));
        }
    } else {
        lines.push(Line::from(Span::styled(" --", dim_style)));
        lines.push(Line::from(Span::styled(format!(" {}", sep), sep_style)));
        lines.push(Line::from(Span::styled(" --", dim_style)));
        lines.push(Line::from(Span::styled(format!(" {}", sep), sep_style)));
        lines.push(Line::from(Span::styled(" --", dim_style)));
    }

    frame.render_widget(Paragraph::new(lines), inner);
    frame.render_widget(block, area);
}

fn render_content(frame: &mut Frame, area: Rect, app: &mut App, full_area: Rect) {
    let [month_hdr_area, day_hdr_area, grid_area, hover_area, nav_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(area);

    let title_style = Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD);
    let ad_range_style = Style::default().fg(app.theme.secondary).add_modifier(Modifier::ITALIC);
    let bs_title = format!(" {} {} ", calendar::english_month_name(app.view_month), app.view_year,);
    let ad_title = format!(" {} ", app.ad_range_str);

    let total_w = area.width as usize;
    let bs_w = bs_title.len();
    let pad_w = total_w.saturating_sub(bs_w).saturating_sub(ad_title.len());
    let month_line = Line::from(vec![
        Span::styled(bs_title, title_style),
        Span::styled(" ".repeat(pad_w), Style::default()),
        Span::styled(ad_title, ad_range_style),
    ]);
    frame.render_widget(Paragraph::new(month_line).alignment(Alignment::Left), month_hdr_area);

    let cell_w = (area.width.saturating_sub(2) / 7).clamp(3, 12) as usize;
    let hdr = Style::default().fg(app.theme.secondary).add_modifier(Modifier::BOLD);
    let sat_hdr = Style::default().fg(app.theme.error).add_modifier(Modifier::BOLD);
    let header_spans: Vec<Span> = DAY_HEADER
        .iter()
        .enumerate()
        .map(|(i, &name)| {
            let s = format!("{:^w$}", name, w = cell_w);
            if i == 6 { Span::styled(s, sat_hdr) } else { Span::styled(s, hdr) }
        })
        .collect();
    frame.render_widget(
        Paragraph::new(Line::from(header_spans)).alignment(Alignment::Center),
        day_hdr_area,
    );

    let ad_style =
        Style::default().fg(app.theme.secondary).add_modifier(Modifier::DIM | Modifier::ITALIC);
    let sat_style = Style::default().fg(app.theme.error);
    let sun_style = Style::default().fg(app.theme.warning).add_modifier(Modifier::BOLD);
    let hover_style =
        Style::default().fg(app.theme.bg).bg(app.theme.secondary).add_modifier(Modifier::BOLD);
    let hover_ad_style =
        Style::default().fg(app.theme.bg).bg(app.theme.secondary).add_modifier(Modifier::ITALIC);
    let mut grid_lines: Vec<Line> = Vec::new();
    for row in &app.calendar_rows {
        let mut bs_spans = Vec::new();
        let mut ad_spans = Vec::new();
        for cell in &row.cells {
            if cell.day == 0 {
                let s = format!("{:^w$}", "", w = cell_w);
                bs_spans.push(Span::styled(s.clone(), Style::default()));
                ad_spans.push(Span::styled(s, Style::default()));
            } else if cell.is_today {
                let s = format!("{:^w$}", cell.day.to_string(), w = cell_w);
                bs_spans.push(Span::styled(
                    s,
                    Style::default()
                        .fg(app.theme.bg)
                        .bg(app.theme.primary)
                        .add_modifier(Modifier::BOLD),
                ));
                let a = format!("{:^w$}", cell.ad_day.to_string(), w = cell_w);
                ad_spans.push(Span::styled(
                    a,
                    Style::default()
                        .fg(app.theme.bg)
                        .bg(app.theme.primary)
                        .add_modifier(Modifier::ITALIC),
                ));
            } else if cell.is_goto_target {
                let s = format!("{:^w$}", format!("★{}", cell.day), w = cell_w);
                bs_spans.push(Span::styled(
                    s,
                    Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD),
                ));
                let a = format!("{:^w$}", format!("★{}", cell.ad_day), w = cell_w);
                ad_spans.push(Span::styled(
                    a,
                    Style::default().fg(app.theme.primary).add_modifier(Modifier::ITALIC),
                ));
            } else if app.hover_bs_day == Some(cell.day) && app.hover_ad_date.is_some() {
                let s = format!("{:^w$}", cell.day.to_string(), w = cell_w);
                bs_spans.push(Span::styled(s, hover_style));
                let a = format!("{:^w$}", cell.ad_day.to_string(), w = cell_w);
                ad_spans.push(Span::styled(a, hover_ad_style));
            } else if cell.is_saturday {
                let s = format!("{:^w$}", cell.day.to_string(), w = cell_w);
                bs_spans.push(Span::styled(s, sat_style));
                let a = format!("{:^w$}", cell.ad_day.to_string(), w = cell_w);
                ad_spans.push(Span::styled(a, ad_style));
            } else if cell.is_sunday {
                let s = format!("{:^w$}", cell.day.to_string(), w = cell_w);
                bs_spans.push(Span::styled(s, sun_style));
                let a = format!("{:^w$}", cell.ad_day.to_string(), w = cell_w);
                ad_spans.push(Span::styled(a, ad_style));
            } else {
                let s = format!("{:^w$}", cell.day.to_string(), w = cell_w);
                bs_spans.push(Span::styled(s, Style::default().fg(app.theme.fg)));
                let a = format!("{:^w$}", cell.ad_day.to_string(), w = cell_w);
                ad_spans.push(Span::styled(a, ad_style));
            }
        }
        grid_lines.push(Line::from(bs_spans));
        grid_lines.push(Line::from(ad_spans));
    }
    frame.render_widget(Paragraph::new(grid_lines).alignment(Alignment::Center), grid_area);

    let hover_style = Style::default().fg(app.theme.secondary).add_modifier(Modifier::ITALIC);
    let full_hover_area =
        Rect { x: full_area.x, y: hover_area.y, width: full_area.width, height: 1 };
    if let Some(ref ad_date) = app.hover_ad_date {
        let bs_day = app.hover_bs_day.unwrap_or(0);
        let month_name = calendar::english_month_name(app.view_month);
        let text = format!("BS {} {} {}  →  {}", app.view_year, month_name, bs_day, ad_date);
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(text, hover_style)))
                .alignment(Alignment::Center),
            full_hover_area,
        );
    } else {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "hover over a date for AD preview",
                Style::default().fg(app.theme.secondary).add_modifier(Modifier::DIM),
            )))
            .alignment(Alignment::Center),
            full_hover_area,
        );
    }

    let (prev_m, prev_y) = app.prev_month_info();
    let (next_m, next_y) = app.next_month_info();
    let prev_name = format!("← {} {}", calendar::english_month_name(prev_m), prev_y);
    let next_name = format!("{} {} →", calendar::english_month_name(next_m), next_y);
    let nav_style = Style::default().fg(app.theme.secondary).add_modifier(Modifier::BOLD);
    let nav_pad =
        (area.width as usize).saturating_sub(prev_name.len()).saturating_sub(next_name.len());
    let nav_line = Line::from(vec![
        Span::styled(prev_name, nav_style),
        Span::styled(" ".repeat(nav_pad), Style::default()),
        Span::styled(next_name, nav_style),
    ]);
    frame.render_widget(Paragraph::new(nav_line).alignment(Alignment::Left), nav_area);
}
