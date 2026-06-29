use crate::app::App;
use crate::theme;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    frame.render_widget(Clear, area);

    let popup_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" Select Default Theme ")
        .border_style(Style::default().fg(app.theme.primary));

    let inner = popup_block.inner(area);
    frame.render_widget(popup_block, area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(3), Constraint::Length(2)])
        .split(inner);

    let current_line = Line::from(vec![
        Span::styled("Current: ", Style::default().fg(app.theme.secondary)),
        Span::styled(
            app.theme.name.clone(),
            Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD),
        ),
    ]);
    frame.render_widget(Paragraph::new(current_line), layout[0]);

    let items: Vec<ListItem> = theme::THEME_DISPLAY
        .iter()
        .enumerate()
        .map(|(i, &name)| {
            let prefix = if i == app.theme_selector_selected { "▸ " } else { "  " };
            let is_active = name == app.theme.name;
            let suffix = if is_active { "  (active)" } else { "" };
            ListItem::new(Line::from(vec![
                Span::styled(prefix, Style::default().fg(app.theme.primary)),
                Span::styled(name, Style::default().fg(app.theme.fg)),
                Span::styled(suffix, Style::default().fg(app.theme.secondary)),
            ]))
        })
        .collect();

    let list = List::new(items).highlight_style(
        Style::default().bg(app.theme.primary).fg(app.theme.bg).add_modifier(Modifier::BOLD),
    );

    let mut list_state = ListState::default();
    list_state.select(Some(app.theme_selector_selected));
    frame.render_stateful_widget(list, layout[1], &mut list_state);

    let help = Paragraph::new(Line::from(vec![
        Span::styled(" ↑/↓ ", Style::default().fg(app.theme.secondary)),
        Span::styled("navigate", Style::default().fg(app.theme.fg)),
        Span::raw("  "),
        Span::styled("Enter", Style::default().fg(app.theme.success)),
        Span::styled(" set default", Style::default().fg(app.theme.fg)),
        Span::raw("  "),
        Span::styled("Esc/q", Style::default().fg(app.theme.error)),
        Span::styled(" cancel", Style::default().fg(app.theme.fg)),
    ]));
    frame.render_widget(help, layout[2]);
}
