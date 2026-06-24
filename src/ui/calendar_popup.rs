use crate::app::App;
use crate::calendar;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

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
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.primary));
    let inner = block.inner(area);

    let [header_area, grid_area, info_area, bottom_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(3),
    ])
    .areas(inner);

    let [date_area, _spacer, hint_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Length(1)])
            .areas(bottom_area);

    let cell_w = (inner.width.saturating_sub(2) / 7).clamp(3, 12) as usize;

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
                Some(cell) if cell.is_goto_target => {
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
    let goto_info = app
        .goto_date_key
        .map(|(y, m, d)| format!("  ★  Goto: {:04}/{:02}/{:02}", y, m, d))
        .unwrap_or_default();
    let info = if let Some(ref nd) = app.today {
        format!(
            "{} · {} · {}{}",
            nd.format_long(),
            now.format("%a, %b %d, %Y"),
            now.format("%I:%M:%S %p"),
            goto_info,
        )
    } else {
        String::new()
    };
    let bold_line = Style::default()
        .fg(app.theme.secondary)
        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
    let bold = Style::default().fg(app.theme.secondary).add_modifier(Modifier::BOLD);
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(info, bold_line))).alignment(Alignment::Center),
        info_area,
    );

    let date_text = app.today.as_ref().map_or_else(
        || "--/--/--".into(),
        |nd| format!("{:04}/{:02}/{:02} · {}", nd.year, nd.month, nd.day, now.format("%Y/%m/%d")),
    );
    frame.render_widget(
        Paragraph::new(Line::from(vec![Span::styled(date_text, bold_line)]))
            .alignment(Alignment::Center),
        date_area,
    );
    frame.render_widget(
        Paragraph::new(Line::from(vec![Span::styled("h/l · j/k · t · ? · q", bold)]))
            .alignment(Alignment::Center),
        hint_area,
    );

    frame.render_widget(block, area);
}
