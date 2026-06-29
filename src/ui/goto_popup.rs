use crate::app::App;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Go to Date ")
        .title_alignment(Alignment::Center)
        .title_bottom(Line::from(" Enter confirm · Esc/q cancel ").alignment(Alignment::Center))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.primary));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(inner);

    let label =
        Line::from(vec![Span::styled("Enter date:", Style::default().fg(app.theme.secondary))]);
    frame.render_widget(Paragraph::new(label), layout[0]);

    let input_line = Line::from(vec![
        Span::styled("> ", Style::default().fg(app.theme.secondary)),
        Span::styled(
            app.goto_input.clone(),
            Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD),
        ),
        Span::styled("_", Style::default().fg(app.theme.primary)),
    ]);
    frame.render_widget(Paragraph::new(input_line), layout[1]);

    frame.render_widget(Paragraph::new(Line::from("")), layout[2]);

    let hint = Line::from(vec![
        Span::styled("Format: ", Style::default().fg(app.theme.secondary)),
        Span::styled(
            "YYYY",
            Style::default().fg(app.theme.bg).bg(app.theme.primary).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" or ", Style::default().fg(app.theme.fg)),
        Span::styled(
            "YYYY-MM",
            Style::default().fg(app.theme.bg).bg(app.theme.primary).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" or ", Style::default().fg(app.theme.fg)),
        Span::styled(
            "YYYY-MM-DD",
            Style::default().fg(app.theme.bg).bg(app.theme.primary).add_modifier(Modifier::BOLD),
        ),
    ]);
    frame.render_widget(Paragraph::new(hint), layout[3]);

    if let Some(ref err) = app.goto_error {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                err.clone(),
                Style::default().fg(app.theme.error),
            )))
            .alignment(Alignment::Center),
            layout[4],
        );
    }
}
