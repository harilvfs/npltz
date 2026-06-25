use crate::app::App;
use crate::calendar;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

use super::DAY_HEADER;

const SIDEBAR_WIDTH: u16 = 24;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Nepali Calendar Patro ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.primary));
    let inner = block.inner(area);

    let [main_area, info_area, hint_area] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(1), Constraint::Length(1)])
            .areas(inner);

    let sb_w = SIDEBAR_WIDTH.min(inner.width / 3);
    let [sidebar_area, content_area] =
        Layout::horizontal([Constraint::Length(sb_w), Constraint::Min(0)]).areas(main_area);

    render_sidebar(frame, sidebar_area, app);
    render_content(frame, content_area, app);

    let now = chrono::Local::now();
    let goto_info = app
        .goto_date_key
        .map(|(y, m, d)| format!("  ★  Goto: {:04}/{:02}/{:02}", y, m, d))
        .unwrap_or_default();
    let info = if let Some(ref nd) = app.today {
        format!("{} · {}{}", nd.format_long(), now.format("%a, %b %d, %Y"), goto_info,)
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
    frame.render_widget(
        Paragraph::new(Line::from(vec![Span::styled("h/l · j/k · t · c · g · ? · q", bold)]))
            .alignment(Alignment::Center),
        hint_area,
    );

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
    let day_num_style = Style::default().fg(app.theme.fg).add_modifier(Modifier::BOLD);
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

    if let Some(ref nd) = app.today {
        let month = calendar::english_month_name(app.view_month);
        let year_str = format!("{} Year", app.view_year);
        let gap = w.saturating_sub(2 + month.len() + year_str.len());
        lines.push(Line::from(vec![
            Span::styled(" ", label_style),
            Span::styled(month, value_style),
            Span::styled(" ".repeat(gap), Style::default()),
            Span::styled(year_str, value_style),
        ]));

        lines.push(Line::from(Span::styled(format!(" {}", sep), sep_style)));

        let day_name_str = nd.day_name();
        let pad_day_name = w.saturating_sub(day_name_str.len()) / 2;
        lines.push(Line::from(vec![
            Span::styled(" ".repeat(pad_day_name), Style::default()),
            Span::styled(day_name_str, label_style),
        ]));
        let day_label = "day";
        let pad_label = w.saturating_sub(day_label.len()) / 2;
        lines.push(Line::from(vec![
            Span::styled(" ".repeat(pad_label), Style::default()),
            Span::styled(day_label, dim_style),
        ]));
        let day_str = nd.day.to_string();
        let pad_num = w.saturating_sub(day_str.len()) / 2;
        lines.push(Line::from(vec![
            Span::styled(" ".repeat(pad_num), Style::default()),
            Span::styled(day_str, day_num_style),
        ]));

        lines.push(Line::from(Span::styled(format!(" {}", sep), sep_style)));

        let time_str = now.format("%I:%M:%S %p").to_string();
        let pad_time = w.saturating_sub(time_str.len()) / 2;
        lines.push(Line::from(vec![
            Span::styled(" ".repeat(pad_time), Style::default()),
            Span::styled(time_str, dim_style),
        ]));
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

fn render_content(frame: &mut Frame, area: Rect, app: &App) {
    let [month_hdr_area, day_hdr_area, grid_area, nav_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
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
