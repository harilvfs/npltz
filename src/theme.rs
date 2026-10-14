use ratatui::style::{Color, Modifier, Style};

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
    pub success:   Color,
    pub warning:   Color,
    pub error:     Color,
    /// Render without colors (NO_COLOR / --no-color).
    pub plain:     bool,
}

impl Theme {
    /// Strong highlight: the currently selected / today cell, list selection.
    ///
    /// In plain mode colors are all `Reset`, so contrast has to come from
    /// reverse video instead of a foreground/background pair.
    pub fn hl(&self) -> Style {
        if self.plain {
            Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD)
        } else {
            Style::default().fg(self.bg).bg(self.primary).add_modifier(Modifier::BOLD)
        }
    }

    /// Softer highlight: hover, secondary accent on a filled background.
    pub fn hl_soft(&self) -> Style {
        if self.plain {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default().fg(self.bg).bg(self.secondary)
        }
    }

    /// A "key badge" style (text sitting on a colored pill).
    pub fn badge(&self, color: Color) -> Style {
        if self.plain {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(self.bg).bg(color).add_modifier(Modifier::BOLD)
        }
    }

    /// Strip every color, keeping the layout and modifiers intact.
    pub fn into_plain(mut self) -> Self {
        self.plain = true;
        self.bg = Color::Reset;
        self.fg = Color::Reset;
        self.primary = Color::Reset;
        self.secondary = Color::Reset;
        self.success = Color::Reset;
        self.warning = Color::Reset;
        self.error = Color::Reset;
        self
    }
}

/// `NO_COLOR` spec: present and non-empty disables color.
pub fn no_color_requested() -> bool {
    std::env::var_os("NO_COLOR").is_some_and(|v| !v.is_empty())
}

impl Theme {
    pub fn catppuccin_mocha() -> Self {
        Self {
            name:      "Catppuccin Mocha".into(),
            bg:        Color::Rgb(30, 30, 46),
            fg:        Color::Rgb(205, 214, 244),
            primary:   Color::Rgb(137, 180, 250),
            secondary: Color::Rgb(173, 216, 190),
            success:   Color::Rgb(166, 227, 161),
            warning:   Color::Rgb(250, 179, 135),
            error:     Color::Rgb(243, 139, 168),
            plain:     false,
        }
    }

    pub fn dracula() -> Self {
        Self {
            name:      "Dracula".into(),
            bg:        Color::Rgb(40, 42, 54),
            fg:        Color::Rgb(248, 248, 242),
            primary:   Color::Rgb(189, 147, 249),
            secondary: Color::Rgb(139, 233, 253),
            success:   Color::Rgb(80, 250, 123),
            warning:   Color::Rgb(241, 250, 140),
            error:     Color::Rgb(255, 85, 85),
            plain:     false,
        }
    }

    pub fn gruvbox() -> Self {
        Self {
            name:      "Gruvbox".into(),
            bg:        Color::Rgb(40, 40, 40),
            fg:        Color::Rgb(235, 219, 178),
            primary:   Color::Rgb(184, 187, 38),
            secondary: Color::Rgb(131, 165, 152),
            success:   Color::Rgb(184, 187, 38),
            warning:   Color::Rgb(214, 93, 14),
            error:     Color::Rgb(204, 36, 29),
            plain:     false,
        }
    }

    pub fn nord() -> Self {
        Self {
            name:      "Nord".into(),
            bg:        Color::Rgb(46, 52, 64),
            fg:        Color::Rgb(216, 222, 233),
            primary:   Color::Rgb(136, 192, 208),
            secondary: Color::Rgb(163, 190, 140),
            success:   Color::Rgb(163, 190, 140),
            warning:   Color::Rgb(235, 203, 139),
            error:     Color::Rgb(191, 97, 106),
            plain:     false,
        }
    }

    pub fn rose_pine() -> Self {
        Self {
            name:      "Rosé Pine".into(),
            bg:        Color::Rgb(25, 23, 36),
            fg:        Color::Rgb(224, 222, 244),
            primary:   Color::Rgb(196, 167, 231),
            secondary: Color::Rgb(156, 207, 216),
            success:   Color::Rgb(49, 116, 143),
            warning:   Color::Rgb(246, 193, 119),
            error:     Color::Rgb(235, 111, 146),
            plain:     false,
        }
    }
}
