use crate::app::App;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
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
