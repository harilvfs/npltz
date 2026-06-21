use ratatui::style::Color;

pub const THEME_NAMES: &[&str] = &["catppuccin-mocha", "dracula", "gruvbox", "nord", "rose-pine"];

pub const THEME_DISPLAY: &[&str] = &["Catppuccin Mocha", "Dracula", "Gruvbox", "Nord", "Rosé Pine"];

pub fn is_valid_theme(name: &str) -> bool {
    THEME_NAMES.contains(&name)
}

pub fn display_name(name: &str) -> &'static str {
    THEME_NAMES
        .iter()
        .position(|&t| t == name)
        .map(|i| THEME_DISPLAY[i])
        .unwrap_or("Catppuccin Mocha")
}

pub fn from_name(name: &str) -> Theme {
    match name {
        "dracula" => Theme::dracula(),
        "gruvbox" => Theme::gruvbox(),
        "nord" => Theme::nord(),
        "rose-pine" => Theme::rose_pine(),
        _ => Theme::catppuccin_mocha(),
    }
}

pub struct Theme {
    pub name:      String,
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
    pub fn catppuccin_mocha() -> Self {
        Self {
            name:      "Catppuccin Mocha".into(),
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

    pub fn dracula() -> Self {
        Self {
            name:      "Dracula".into(),
            bg:        Color::Rgb(40, 42, 54),
            fg:        Color::Rgb(248, 248, 242),
            primary:   Color::Rgb(189, 147, 249),
            secondary: Color::Rgb(139, 233, 253),
            accent:    Color::Rgb(255, 121, 198),
            success:   Color::Rgb(80, 250, 123),
            warning:   Color::Rgb(241, 250, 140),
            error:     Color::Rgb(255, 85, 85),
        }
    }

    pub fn gruvbox() -> Self {
        Self {
            name:      "Gruvbox".into(),
            bg:        Color::Rgb(40, 40, 40),
            fg:        Color::Rgb(235, 219, 178),
            primary:   Color::Rgb(184, 187, 38),
            secondary: Color::Rgb(131, 165, 152),
            accent:    Color::Rgb(211, 134, 155),
            success:   Color::Rgb(152, 151, 26),
            warning:   Color::Rgb(214, 93, 14),
            error:     Color::Rgb(204, 36, 29),
        }
    }

    pub fn nord() -> Self {
        Self {
            name:      "Nord".into(),
            bg:        Color::Rgb(46, 52, 64),
            fg:        Color::Rgb(216, 222, 233),
            primary:   Color::Rgb(136, 192, 208),
            secondary: Color::Rgb(163, 190, 140),
            accent:    Color::Rgb(180, 142, 173),
            success:   Color::Rgb(163, 190, 140),
            warning:   Color::Rgb(235, 203, 139),
            error:     Color::Rgb(191, 97, 106),
        }
    }

    pub fn rose_pine() -> Self {
        Self {
            name:      "Rosé Pine".into(),
            bg:        Color::Rgb(25, 23, 36),
            fg:        Color::Rgb(224, 222, 244),
            primary:   Color::Rgb(196, 167, 231),
            secondary: Color::Rgb(156, 207, 216),
            accent:    Color::Rgb(235, 111, 146),
            success:   Color::Rgb(62, 143, 176),
            warning:   Color::Rgb(246, 193, 119),
            error:     Color::Rgb(235, 111, 146),
        }
    }
}
