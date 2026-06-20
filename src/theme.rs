use ratatui::style::Color;

#[allow(dead_code)]
pub struct Theme {
    pub bg:        Color,
    pub fg:        Color,
    pub primary:   Color,
    pub secondary: Color,
    pub accent:    Color,
    pub success:   Color,
    pub warning:   Color,
    pub error:     Color,
}

impl Theme {
    pub fn default() -> Self {
        Self {
            bg:        Color::Rgb(30, 30, 46),
            fg:        Color::Rgb(205, 214, 244),
            primary:   Color::Rgb(137, 180, 250),
            secondary: Color::Rgb(173, 216, 190),
            accent:    Color::Rgb(243, 139, 168),
            success:   Color::Rgb(166, 227, 161),
            warning:   Color::Rgb(250, 179, 135),
            error:     Color::Rgb(243, 139, 168),
        }
    }
}
