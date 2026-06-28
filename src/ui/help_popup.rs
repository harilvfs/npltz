use crate::app::App;
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

pub fn render(frame: &mut Frame, area: Rect, app: &App) -> u16 {
    frame.render_widget(Clear, area);

    let bold = Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD);
    let sec = Style::default().fg(app.theme.secondary);
    let warn = Style::default().fg(app.theme.warning).add_modifier(Modifier::BOLD);

    let lines = vec![
        Line::from(""),
        Line::from(vec![Span::styled("  Navigation", bold)]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ←  →      ", warn),
            Span::styled("h  l     Prev / next month", sec),
        ]),
        Line::from(vec![
            Span::styled("  ↑  ↓      ", warn),
            Span::styled("k  j     Prev / next year", sec),
        ]),
        Line::from(vec![Span::styled("  t        ", warn), Span::styled("Jump to today", sec)]),
        Line::from(vec![
            Span::styled("  g        ", warn),
            Span::styled("Go to specific date", sec),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("  Actions", bold)]),
        Line::from(""),
        Line::from(vec![Span::styled("  y        ", warn), Span::styled("Year overview", sec)]),
        Line::from(vec![Span::styled("  c        ", warn), Span::styled("Change theme", sec)]),
        Line::from(vec![Span::styled("  Mouse", bold)]),
        Line::from(""),
        Line::from(vec![Span::styled("  Scroll   ", warn), Span::styled("Prev / next month", sec)]),
        Line::from(""),
        Line::from(vec![Span::styled("  Theme Selector", bold)]),
        Line::from(""),
        Line::from(vec![Span::styled("  j/k      ", warn), Span::styled("Prev / next theme", sec)]),
        Line::from(vec![
            Span::styled("  Enter    ", warn),
            Span::styled("Apply selected theme", sec),
        ]),
        Line::from(vec![Span::styled("  Home     ", warn), Span::styled("First theme", sec)]),
        Line::from(vec![Span::styled("  End      ", warn), Span::styled("Last theme", sec)]),
        Line::from(""),
        Line::from(vec![Span::styled("  General", bold)]),
        Line::from(""),
        Line::from(vec![Span::styled("  ?        ", warn), Span::styled("Show this help", sec)]),
        Line::from(vec![Span::styled("  q        ", warn), Span::styled("Quit", sec)]),
        Line::from(vec![
            Span::styled("  Esc      ", warn),
            Span::styled("Close popups / quit", sec),
        ]),
        Line::from(vec![Span::styled("  PgUp     ", warn), Span::styled("Scroll help up", sec)]),
        Line::from(vec![Span::styled("  PgDown   ", warn), Span::styled("Scroll help down", sec)]),
    ];

    let tmp = Block::default().borders(Borders::ALL);
    let inner = tmp.inner(area);

    let content_area = Rect {
        x:      inner.x + 1,
        y:      inner.y,
        width:  inner.width.saturating_sub(2),
        height: inner.height.saturating_sub(2),
    };

    let content_height = lines.len() as u16;
    let max_scroll = content_height.saturating_sub(content_area.height);

    let title = if max_scroll > 0 {
        format!(" Help  {}/{}  PgUp/PgDn scroll ", app.help_scroll.min(max_scroll), max_scroll)
    } else {
        " Help ".to_string()
    };

    let block = Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.primary));

    frame.render_widget(block, area);

    let help_paragraph = Paragraph::new(lines).scroll((app.help_scroll, 0));
    frame.render_widget(help_paragraph, content_area);

    max_scroll
}
