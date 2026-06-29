use crate::app::App;
use crate::theme::Theme;
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

#[derive(Clone, Copy)]
enum Kind {
    Header,
    Key,
    NavKey,
    Body,
}

const HELP_LINES: &[(&[&str], &[Kind])] = &[
    (&["Navigation"], &[Kind::Header]),
    (&[], &[]),
    (&[" h/l ", " ", "Prev / next month"], &[Kind::NavKey, Kind::Body, Kind::Body]),
    (&[], &[]),
    (
        &[" \u{2190}/\u{2192} ", " ", "Prev / next month (arrow keys)"],
        &[Kind::NavKey, Kind::Body, Kind::Body],
    ),
    (&[], &[]),
    (&[" j/k ", " ", "Prev / next year"], &[Kind::NavKey, Kind::Body, Kind::Body]),
    (&[], &[]),
    (
        &[" \u{2191}/\u{2193} ", " ", "Prev / next year (arrow keys)"],
        &[Kind::NavKey, Kind::Body, Kind::Body],
    ),
    (&[], &[]),
    (&[" t ", " ", "Jump to today"], &[Kind::Key, Kind::Body, Kind::Body]),
    (&[], &[]),
    (&[" g ", " ", "Go to specific date"], &[Kind::Key, Kind::Body, Kind::Body]),
    (&[], &[]),
    (&["Actions"], &[Kind::Header]),
    (&[], &[]),
    (&[" y ", " ", "Year overview"], &[Kind::Key, Kind::Body, Kind::Body]),
    (&[], &[]),
    (&[" c ", " ", "Change theme"], &[Kind::Key, Kind::Body, Kind::Body]),
    (&[], &[]),
    (&["Mouse"], &[Kind::Header]),
    (&[], &[]),
    (&[" Scroll ", " ", "Prev / next month"], &[Kind::NavKey, Kind::Body, Kind::Body]),
    (&[], &[]),
    (&["Theme Selector"], &[Kind::Header]),
    (&[], &[]),
    (&[" j/k ", " ", "Prev / next theme"], &[Kind::NavKey, Kind::Body, Kind::Body]),
    (&[], &[]),
    (&[" Enter ", " ", "Apply selected theme"], &[Kind::Key, Kind::Body, Kind::Body]),
    (&[], &[]),
    (&[" Home ", " ", "First theme"], &[Kind::NavKey, Kind::Body, Kind::Body]),
    (&[], &[]),
    (&[" End ", " ", "Last theme"], &[Kind::NavKey, Kind::Body, Kind::Body]),
    (&[], &[]),
    (&["General"], &[Kind::Header]),
    (&[], &[]),
    (&[" ? ", " ", "Show this help"], &[Kind::Key, Kind::Body, Kind::Body]),
    (&[], &[]),
    (&[" q ", " ", "Quit"], &[Kind::Key, Kind::Body, Kind::Body]),
    (&[], &[]),
    (&[" Esc ", " ", "Close popups / quit"], &[Kind::Key, Kind::Body, Kind::Body]),
    (&[], &[]),
    (&[" PgUp ", " ", "Scroll help up"], &[Kind::NavKey, Kind::Body, Kind::Body]),
    (&[], &[]),
    (&[" PgDown ", " ", "Scroll help down"], &[Kind::NavKey, Kind::Body, Kind::Body]),
    (&[], &[]),
    (&[" Scroll (in help) ", " ", "Scroll help content"], &[Kind::NavKey, Kind::Body, Kind::Body]),
];

fn style_for(kind: Kind, theme: &Theme) -> Style {
    match kind {
        Kind::Header => Style::default().fg(theme.warning).add_modifier(Modifier::BOLD),
        Kind::NavKey => Style::default().bg(theme.primary).fg(theme.bg),
        Kind::Key => Style::default().bg(theme.success).fg(theme.bg),
        Kind::Body => Style::default(),
    }
}

fn build_help_content(theme: &Theme) -> Vec<Line<'static>> {
    HELP_LINES
        .iter()
        .map(|(texts, kinds)| {
            if texts.is_empty() {
                Line::from("")
            } else {
                let spans: Vec<_> = texts
                    .iter()
                    .zip(kinds.iter())
                    .map(|(t, k)| Span::styled((*t).to_string(), style_for(*k, theme)))
                    .collect();
                Line::from(spans)
            }
        })
        .collect()
}

pub fn render(frame: &mut Frame, area: Rect, app: &App) -> u16 {
    frame.render_widget(Clear, area);

    let popup_block = Block::default()
        .title("Keyboard Shortcuts")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.primary));

    let inner = popup_block.inner(area);
    frame.render_widget(popup_block, area);

    let content_area = Rect {
        x:      inner.x + 1,
        y:      inner.y,
        width:  inner.width.saturating_sub(2),
        height: inner.height.saturating_sub(2),
    };

    let help_content = build_help_content(&app.theme);
    let content_height = help_content.len() as u16;
    let max_scroll = content_height.saturating_sub(content_area.height);

    let help_paragraph =
        Paragraph::new(help_content).block(Block::default()).scroll((app.help_scroll, 0));
    frame.render_widget(help_paragraph, content_area);

    let footer_area = Rect {
        x:      area.x + 1,
        y:      area.y + area.height.saturating_sub(2),
        width:  area.width.saturating_sub(2),
        height: 1,
    };

    let scroll_status = if max_scroll > 0 {
        format!("{}/{}", app.help_scroll.min(max_scroll), max_scroll)
    } else {
        "No scroll needed".to_string()
    };

    let footer = Paragraph::new(Line::from(vec![Span::styled(
        scroll_status,
        Style::default().fg(app.theme.secondary),
    )]))
    .alignment(Alignment::Center);

    frame.render_widget(footer, footer_area);

    max_scroll
}
