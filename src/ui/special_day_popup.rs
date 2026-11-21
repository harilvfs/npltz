use crate::app::App;
use crate::calendar::{self, SpecialDay};
use chrono::Datelike;
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};

fn accent(theme: &crate::theme::Theme, special: SpecialDay) -> Color {
    match special {
        SpecialDay::NepaliNewYear => theme.warning,
        SpecialDay::GregorianNewYear => theme.secondary,
        SpecialDay::DeveloperBirthday => theme.error,
    }
}

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let Some(day) = app.selected_day else { return };
    let Some(special) = calendar::special_day(app.view_year, app.view_month, day) else {
        return;
    };
    let Some(ad) = calendar::bs_to_ad(app.view_year, app.view_month, day) else {
        return;
    };

    frame.render_widget(Clear, area);

    let accent = accent(&app.theme, special);
    let block = Block::default()
        .title(" Celebration ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(accent));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width == 0 || inner.height == 0 {
        return;
    }

    let date = format!(
        "{} {} \u{b7} {}",
        calendar::english_month_name(app.view_month),
        day,
        ad.format("%b %d, %Y")
    );

    let lines = vec![
        Line::from(Span::styled(
            special.title(ad.year()),
            Style::default().fg(app.theme.primary).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(date, Style::default().fg(app.theme.secondary))),
        Line::from(""),
        Line::from(Span::styled(special.greeting(), Style::default().fg(app.theme.fg))),
        Line::from(Span::styled(
            "Enter / Esc / click to close",
            Style::default().fg(app.theme.secondary).add_modifier(Modifier::DIM),
        )),
    ];

    frame.render_widget(
        Paragraph::new(lines).alignment(Alignment::Center).wrap(Wrap { trim: true }),
        inner,
    );
}
