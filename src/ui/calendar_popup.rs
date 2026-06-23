use crate::app::App;
use crate::calendar;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};

use super::DAY_HEADER;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    frame.render_widget(Clear, area);

    let block_title = format!(
        " {} {}  ·  {} ",
        calendar::english_month_name(app.view_month),
        app.view_year,
        app.ad_range_str,
    );
    let block = Block::default()
        .title(block_title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.primary));
    let inner = block.inner(area);

    let [header_area, grid_area, info_area, nav_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(4),
        Constraint::Length(1),
    ])
    .areas(inner);

    let cell_w = (inner.width.saturating_sub(2) / 7).clamp(3, 10) as usize;

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
        header_area,
    );

    let ad_style =
        Style::default().fg(app.theme.secondary).add_modifier(Modifier::DIM | Modifier::ITALIC);
    let sat_style = Style::default().fg(app.theme.error);
    let mut grid_lines: Vec<Line> = Vec::new();
    for row in &app.calendar_rows {
        let mut bs_spans = Vec::new();
        let mut ad_spans = Vec::new();
        for cell_opt in &row.cells {
            match cell_opt {
                Some(cell) if cell.is_today => {
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
                }
                Some(cell) if cell.is_saturday => {
                    let s = format!("{:^w$}", cell.day.to_string(), w = cell_w);
                    bs_spans.push(Span::styled(s, sat_style));
                    let a = format!("{:^w$}", cell.ad_day.to_string(), w = cell_w);
                    ad_spans.push(Span::styled(a, ad_style));
                }
                Some(cell) => {
                    let s = format!("{:^w$}", cell.day.to_string(), w = cell_w);
                    bs_spans.push(Span::styled(s, Style::default().fg(app.theme.fg)));
                    let a = format!("{:^w$}", cell.ad_day.to_string(), w = cell_w);
                    ad_spans.push(Span::styled(a, ad_style));
                }
                None => {
                    let s = format!("{:^w$}", "", w = cell_w);
                    bs_spans.push(Span::styled(s.clone(), Style::default()));
                    ad_spans.push(Span::styled(s, Style::default()));
                }
            }
        }
        grid_lines.push(Line::from(bs_spans));
        grid_lines.push(Line::from(ad_spans));
    }
    frame.render_widget(Paragraph::new(grid_lines).alignment(Alignment::Center), grid_area);

    let now = chrono::Local::now();
    let label = Style::default().fg(app.theme.secondary).add_modifier(Modifier::BOLD);

    let nd = app.today.as_ref();
    let nepali_long = nd.map_or_else(|| "N/A".into(), |n| n.format_long());
    let nepali_num =
        nd.map_or_else(|| "N/A".into(), |n| format!("{:04}/{:02}/{:02}", n.year, n.month, n.day));
    let english_num = now.format("%Y/%m/%d").to_string();

    let info_lines = vec![
        Line::from(vec![
            Span::styled("Nepali: ", label),
            Span::raw(format!("{}  ({})", nepali_num, nepali_long)),
        ]),
        Line::from(vec![
            Span::styled("English:", label),
            Span::raw(format!(" {}  ({})", english_num, now.format("%A, %B %d, %Y"))),
        ]),
        Line::from(vec![
            Span::styled("Time:   ", label),
            Span::styled(
                now.format(" %I:%M:%S %p").to_string(),
                Style::default().fg(app.theme.warning).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(info_lines).wrap(Wrap { trim: false }).alignment(Alignment::Center),
        info_area,
    );

    let nav_style = Style::default().fg(app.theme.secondary);
    let nav = Line::from(vec![Span::styled(
        "  h/l: month  |  j/k: year  |  g: goto date  |  t: today  |  c: theme  |  q: quit",
        nav_style,
    )])
    .alignment(Alignment::Center);
    frame.render_widget(nav, nav_area);

    frame.render_widget(block, area);
}
