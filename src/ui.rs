use crate::app::App;
use crate::calendar;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_w = r.width * percent_x / 100;
    let popup_h = r.height * percent_y / 100;
    let offset_x = (r.width - popup_w) / 2;
    let offset_y = (r.height - popup_h) / 2;
    Rect {
        x: r.x + offset_x,
        y: r.y + offset_y,
        width: popup_w,
        height: popup_h,
    }
}

const DAY_HEADER: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let popup_area = centered_rect(80, 80, area);
    render_calendar_popup(frame, popup_area, app);

    let status_area = Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(1),
        width: area.width,
        height: 1,
    };
    render_status_bar(frame, status_area, app);
}

fn render_calendar_popup(frame: &mut Frame, area: Rect, app: &App) {
    frame.render_widget(Clear, area);

    let block_title = format!(
        " {} {} ",
        calendar::english_month_name(app.view_month),
        app.view_year
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

    let cell_w = 10usize;

    let hdr = Style::default()
        .fg(app.theme.secondary)
        .add_modifier(Modifier::BOLD);
    let header_line = Line::from(
        DAY_HEADER
            .iter()
            .map(|&name| Span::styled(format!("{:^w$}", name, w = cell_w), hdr))
            .collect::<Vec<_>>(),
    );
    frame.render_widget(
        Paragraph::new(header_line).alignment(Alignment::Center),
        header_area,
    );
    let mut grid_lines: Vec<Line> = Vec::new();
    for row in &app.calendar_rows {
        let mut spans = Vec::new();
        for cell_opt in &row.cells {
            match cell_opt {
                Some(cell) if cell.is_today => {
                    let s = format!("{:^w$}", cell.day.to_string(), w = cell_w);
                    spans.push(Span::styled(
                        s,
                        Style::default()
                            .fg(app.theme.bg)
                            .bg(app.theme.primary)
                            .add_modifier(Modifier::BOLD),
                    ));
                }
                Some(cell) => {
                    let s = format!("{:^w$}", cell.day.to_string(), w = cell_w);
                    spans.push(Span::styled(s, Style::default().fg(app.theme.fg)));
                }
                None => {
                    let s = format!("{:^w$}", "", w = cell_w);
                    spans.push(Span::styled(s, Style::default()));
                }
            }
        }
        grid_lines.push(Line::from(spans));
        grid_lines.push(Line::from(""));
    }
    frame.render_widget(
        Paragraph::new(grid_lines).alignment(Alignment::Center),
        grid_area,
    );

    let now = chrono::Local::now();
    let label = Style::default()
        .fg(app.theme.secondary)
        .add_modifier(Modifier::BOLD);

    let today_str = app
        .today
        .as_ref()
        .map(|nd| nd.format_long())
        .unwrap_or_else(|| "N/A".into());

    let info_lines = vec![
        Line::from(vec![Span::styled("Nepali: ", label), Span::raw(today_str)]),
        Line::from(vec![
            Span::styled("English:", label),
            Span::raw(now.format(" %A, %B %d, %Y").to_string()),
        ]),
        Line::from(vec![
            Span::styled("Clock:  ", label),
            Span::styled(
                now.format(" %I:%M:%S %p").to_string(),
                Style::default()
                    .fg(app.theme.warning)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(info_lines)
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center),
        info_area,
    );

    let nav_style = Style::default().fg(app.theme.secondary);
    let nav = Line::from(vec![Span::styled(
        "  <-  |  ->  |  t: today  |  q: quit",
        nav_style,
    )])
    .alignment(Alignment::Center);
    frame.render_widget(nav, nav_area);

    frame.render_widget(block, area);
}

fn render_status_bar(frame: &mut Frame, area: Rect, app: &App) {
    let now = chrono::Local::now();
    let time_str = now.format("%I:%M:%S %p").to_string();

    let left = format!(" npltz v{} ", env!("CARGO_PKG_VERSION"));
    let right = format!(" {} ", time_str);

    let bar = Paragraph::new(Line::from(vec![
        Span::styled(
            left,
            Style::default().fg(app.theme.bg).bg(app.theme.primary),
        ),
        Span::styled(
            "  ",
            Style::default().fg(app.theme.bg).bg(app.theme.primary),
        ),
        Span::styled(
            right,
            Style::default().fg(app.theme.bg).bg(app.theme.secondary),
        ),
    ]));

    frame.render_widget(bar, area);
}
