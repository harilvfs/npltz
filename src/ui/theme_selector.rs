use crate::app::App;
use crate::theme;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
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
