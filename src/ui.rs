use crate::app::{App, AppMode};
use crate::{calendar, theme};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap,
};

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_w = r.width * percent_x / 100;
    let popup_h = r.height * percent_y / 100;
    let offset_x = (r.width - popup_w) / 2;
    let offset_y = (r.height - popup_h) / 2;
    Rect { x: r.x + offset_x, y: r.y + offset_y, width: popup_w, height: popup_h }
}

const DAY_HEADER: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let popup_area = centered_rect(80, 80, area);
    render_calendar_popup(frame, popup_area, app);

    let status_area = Rect {
        x:      area.x,
        y:      area.y + area.height.saturating_sub(1),
        width:  area.width,
        height: 1,
    };
    render_status_bar(frame, status_area, app);

    match app.mode {
        AppMode::ThemeSelector => {
            let sel_area = centered_rect(40, 30, area);
            render_theme_selector(frame, sel_area, app);
        }
        AppMode::Goto => {
            let goto_area = centered_rect(40, 20, area);
            render_goto_popup(frame, goto_area, app);
        }
        AppMode::Normal => {}
    }
}

fn render_calendar_popup(frame: &mut Frame, area: Rect, app: &App) {
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

    let cell_w = 10usize;

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

fn render_theme_selector(frame: &mut Frame, area: Rect, app: &App) {
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Select Default Theme ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.primary));
    let inner = block.inner(area);

    let [current_area, list_area, help_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)])
            .areas(inner);

    let current_line = Line::from(vec![
        Span::styled("  Current: ", Style::default().fg(app.theme.secondary)),
        Span::styled(
            app.theme.name.clone(),
            Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD),
        ),
    ]);
    frame.render_widget(Paragraph::new(current_line), current_area);

    let items: Vec<ListItem> = theme::THEME_DISPLAY
        .iter()
        .enumerate()
        .map(|(i, &name)| {
            let prefix = if i == app.theme_selector_selected { "▸ " } else { "  " };
            let is_active = name == app.theme.name;
            let suffix = if is_active { "  (active)" } else { "" };
            let content = Line::from(vec![
                Span::styled(prefix, Style::default().fg(app.theme.primary)),
                Span::styled(name, Style::default().fg(app.theme.fg)),
                Span::styled(suffix, Style::default().fg(app.theme.secondary)),
            ]);
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .highlight_style(
            Style::default().bg(app.theme.primary).fg(app.theme.bg).add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("");

    let mut list_state = ListState::default().with_selected(Some(app.theme_selector_selected));
    frame.render_stateful_widget(list, list_area, &mut list_state);

    let help = Line::from(vec![
        Span::styled(" j/k ", Style::default().fg(app.theme.success)),
        Span::styled("navigate", Style::default().fg(app.theme.fg)),
        Span::styled("  Enter ", Style::default().fg(app.theme.success)),
        Span::styled("apply", Style::default().fg(app.theme.fg)),
        Span::styled("  q/esc ", Style::default().fg(app.theme.error)),
        Span::styled("cancel", Style::default().fg(app.theme.fg)),
    ])
    .alignment(Alignment::Center);
    frame.render_widget(help, help_area);

    frame.render_widget(block, area);
}

fn render_goto_popup(frame: &mut Frame, area: Rect, app: &App) {
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Go to Date ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.primary));
    let inner = block.inner(area);

    let [input_area, hint_area, error_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(1), Constraint::Length(1)])
            .areas(inner);

    let input_line = Line::from(vec![
        Span::styled("> ", Style::default().fg(app.theme.secondary)),
        Span::styled(
            app.goto_input.clone(),
            Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD),
        ),
        Span::styled("_", Style::default().fg(app.theme.primary)),
    ]);
    frame.render_widget(Paragraph::new(input_line), input_area);

    let hint = Line::from(vec![Span::styled(
        " YYYY, YYYY-MM, or YYYY-MM-DD",
        Style::default().fg(app.theme.secondary),
    )]);
    frame.render_widget(Paragraph::new(hint), hint_area);

    if let Some(ref err) = app.goto_error {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                err.clone(),
                Style::default().fg(app.theme.error),
            )))
            .alignment(Alignment::Center),
            error_area,
        );
    }

    frame.render_widget(block, area);
}

fn render_status_bar(frame: &mut Frame, area: Rect, app: &App) {
    let text = format!(" npltz v{} ", env!("CARGO_PKG_VERSION"));
    let bar = Paragraph::new(Line::from(vec![Span::styled(
        text,
        Style::default().fg(app.theme.bg).bg(app.theme.primary),
    )]))
    .alignment(Alignment::Center);
    frame.render_widget(bar, area);
}
