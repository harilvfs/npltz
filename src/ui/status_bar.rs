use crate::app::App;
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let text = format!(" npltz v{} ", env!("CARGO_PKG_VERSION"));
    let bar = Paragraph::new(Line::from(vec![Span::styled(
        text,
        Style::default().fg(app.theme.bg).bg(app.theme.primary),
    )]))
    .alignment(Alignment::Center);
    frame.render_widget(bar, area);
}
