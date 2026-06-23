use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};

pub fn render(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Warning ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(ratatui::style::Color::Red));
    let inner = block.inner(area);

    let lines = vec![
        Line::from(Span::styled(
            "Terminal too small",
            Style::default().fg(ratatui::style::Color::Red).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::raw("")),
        Line::from(Span::raw("The calendar may not display correctly")),
        Line::from(Span::raw("at this size.")),
        Line::from(Span::raw("")),
        Line::from(Span::raw("Try landscape mode or a larger terminal.")),
        Line::from(Span::raw("")),
        Line::from(Span::styled(
            "Press Enter to continue",
            Style::default().fg(ratatui::style::Color::DarkGray),
        )),
    ];

    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(lines).alignment(Alignment::Center).wrap(Wrap { trim: false }),
        inner,
    );
    frame.render_widget(block, area);
}
