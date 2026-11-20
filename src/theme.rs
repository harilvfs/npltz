use ratatui::style::{Color, Modifier, Style};

pub const THEME_NAMES: &[&str] = &[
    "catppuccin-mocha",
    "dracula",
    "everforest",
    "gruvbox",
    "light",
    "nord",
    "plain-dark",
    "rose-pine",
    "warm-reading",
];

pub const THEME_DISPLAY: &[&str] = &[
    "Catppuccin Mocha",
    "Dracula",
    "Everforest",
    "Gruvbox",
    "Light",
    "Nord",
    "Plain dark",
    "Rosé Pine",
    "Warm reading",
];

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
        "everforest" => Theme::everforest(),
        "gruvbox" => Theme::gruvbox(),
        "light" => Theme::light(),
        "nord" => Theme::nord(),
        "plain-dark" => Theme::plain_dark(),
        "rose-pine" => Theme::rose_pine(),
        "warm-reading" => Theme::warm_reading(),
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
    /// In plain mode every color is `Reset`, so contrast comes from reverse video.
    pub fn hl(&self) -> Style {
        if self.plain {
            Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD)
        } else {
            Style::default().fg(self.bg).bg(self.primary).add_modifier(Modifier::BOLD)
        }
    }

    pub fn hl_soft(&self) -> Style {
        if self.plain {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default().fg(self.bg).bg(self.secondary)
        }
    }

    pub fn badge(&self, color: Color) -> Style {
        if self.plain {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(self.bg).bg(color).add_modifier(Modifier::BOLD)
        }
    }

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
    pub fn everforest() -> Self {
        Self {
            name:      "Everforest".into(),
            bg:        Color::Rgb(45, 53, 59),
            fg:        Color::Rgb(211, 198, 170),
            primary:   Color::Rgb(167, 192, 128),
            secondary: Color::Rgb(127, 187, 179),
            success:   Color::Rgb(167, 192, 128),
            warning:   Color::Rgb(230, 152, 117),
            error:     Color::Rgb(230, 126, 128),
            plain:     false,
        }
    }

    pub fn light() -> Self {
        Self {
            name:      "Light".into(),
            bg:        Color::Rgb(246, 246, 244),
            fg:        Color::Rgb(27, 27, 31),
            primary:   Color::Rgb(63, 91, 158),
            secondary: Color::Rgb(94, 110, 126),
            success:   Color::Rgb(46, 125, 79),
            warning:   Color::Rgb(138, 90, 0),
            error:     Color::Rgb(179, 38, 30),
            plain:     false,
        }
    }

    pub fn plain_dark() -> Self {
        Self {
            name:      "Plain dark".into(),
            bg:        Color::Rgb(0, 0, 0),
            fg:        Color::Rgb(230, 230, 230),
            primary:   Color::Rgb(90, 90, 90),
            secondary: Color::Rgb(122, 122, 122),
            success:   Color::Rgb(95, 138, 95),
            warning:   Color::Rgb(176, 138, 64),
            error:     Color::Rgb(176, 90, 90),
            plain:     false,
        }
    }

    pub fn warm_reading() -> Self {
        Self {
            name:      "Warm reading".into(),
            bg:        Color::Rgb(243, 233, 214),
            fg:        Color::Rgb(74, 58, 42),
            primary:   Color::Rgb(156, 90, 50),
            secondary: Color::Rgb(131, 112, 90),
            success:   Color::Rgb(91, 122, 74),
            warning:   Color::Rgb(154, 106, 30),
            error:     Color::Rgb(176, 74, 58),
            plain:     false,
        }
    }
}
