use crate::app::App;
use crate::calendar;
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(format!(" {} Year Overview ", app.view_year))
        .title_alignment(Alignment::Center)
        .title_bottom(Line::from(" h/l year · t today · y close ").alignment(Alignment::Center))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.primary));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 30 || inner.height < 6 {
        let msg = Paragraph::new("Terminal too small")
            .style(Style::default().fg(app.theme.warning))
            .alignment(Alignment::Center);
        frame.render_widget(msg, inner);
        return;
    }

    let cols = 3u16;
    let rows = 4u16;
    let gap_x = 2u16;
    let gap_y = 0u16;
    let usable_w = inner.width.saturating_sub(gap_x * (cols - 1));
    let usable_h = inner.height.saturating_sub(gap_y * (rows - 1));
    let col_w = usable_w / cols;
    let row_h = usable_h / rows;

    let mut weekdays = [0u32; 12];
    let mut days_in_months = [0u32; 12];
    if let Some(first_wd) = calendar::month_start_weekday(app.view_year, 1) {
        let mut wd = first_wd;
        for m in 0..12 {
            weekdays[m] = wd as u32;
            if let Some(d) = calendar::get_days_in_month(app.view_year, (m + 1) as u32) {
                days_in_months[m] = d;
                wd = (wd + d as usize) % 7;
            }
        }
    } else {
        for m in 1..=12 {
            let idx = (m - 1) as usize;
            weekdays[idx] = calendar::month_start_weekday(app.view_year, m).unwrap_or(0) as u32;
            days_in_months[idx] = calendar::get_days_in_month(app.view_year, m).unwrap_or(0);
        }
    }

    for month in 1u32..=12 {
        let col = ((month - 1) % 3) as u16;
        let row = ((month - 1) / 3) as u16;

        let x = inner.x + col * (col_w + gap_x);
        let y = inner.y + row * (row_h + gap_y);
        let cell_area = Rect { x, y, width: col_w, height: row_h };

        render_mini_month(
            frame,
            cell_area,
            app,
            month,
            weekdays[(month - 1) as usize],
            days_in_months[(month - 1) as usize],
        );
    }
}

fn render_mini_month(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    month: u32,
    start_weekday: u32,
    days_in_month: u32,
) {
    if area.width < 10 || area.height < 2 {
        return;
    }

    let month_names = [
        "Baisakh", "Jestha", "Ashadh", "Shrawan", "Bhadra", "Ashwin", "Kartik", "Mangsir", "Poush",
        "Magh", "Falgun", "Chaitra",
    ];

    let title_style = Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD);
    let day_style = Style::default().fg(app.theme.fg);
    let sun_style = Style::default().fg(app.theme.warning);
    let sat_style = Style::default().fg(app.theme.error);
    let today_style =
        Style::default().fg(app.theme.bg).bg(app.theme.primary).add_modifier(Modifier::BOLD);

    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(Span::styled(
        format!(" {} ", month_names[(month - 1) as usize]),
        title_style,
    )));

    if days_in_month == 0 {
        return;
    }

    let mut week_rows: Vec<Vec<u32>> = Vec::new();
    let mut current_week: Vec<u32> = vec![0; start_weekday as usize];

    for d in 1..=days_in_month {
        current_week.push(d);
        if current_week.len() == 7 {
            week_rows.push(current_week);
            current_week = Vec::new();
        }
    }

    if !current_week.is_empty() {
        while current_week.len() < 7 {
            current_week.push(0);
        }
        week_rows.push(current_week);
    }

    let available_lines = area.height.saturating_sub(1) as usize;

    for week in week_rows.iter().take(available_lines) {
        let mut spans = vec![];

        for (i, &day) in week.iter().enumerate() {
            let cell = if day > 0 { format!("{day:2}") } else { "  ".to_string() };

            let is_today = app
                .today
                .as_ref()
                .is_some_and(|td| td.year == app.view_year && td.month == month && td.day == day);

            let style = if is_today {
                today_style
            } else if i == 0 {
                sun_style
            } else if i == 6 {
                sat_style
            } else {
                day_style
            };

            spans.push(Span::styled(format!(" {cell}"), style));
        }

        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}
