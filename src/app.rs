use crate::calendar::{self, NepaliDate, get_days_in_month, month_ad_range, month_start_weekday};
use crate::config::Config;
use crate::theme::Theme;
use crate::{log, theme};
use chrono::{Datelike, Local};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme;

    fn test_app() -> App {
        let theme = theme::from_name("catppuccin-mocha");
        let mut app = App {
            should_quit: false,
            show_small_warning: false,
            mode: AppMode::Normal,
            theme,
            today: None,
            view_year: 2081,
            view_month: 1,
            initial_view_set: true,
            ad_range_str: String::new(),
            calendar_rows: Vec::new(),
            today_key: None,
            theme_selector_selected: 0,
            goto_input: String::new(),
            goto_error: None,
        };
        app.build_view();
        app
    }

    #[test]
    fn test_ad_dates_in_cells() {
        let app = test_app();
        let has_ad_dates = app
            .calendar_rows
            .iter()
            .flat_map(|r| &r.cells)
            .flatten()
            .any(|c| c.ad_day > 0 && c.ad_day <= 31);
        assert!(has_ad_dates);
    }

    #[test]
    fn test_initial_state() {
        let app = test_app();
        assert!(!app.should_quit);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.view_year, 2081);
        assert_eq!(app.view_month, 1);
    }

    #[test]
    fn test_navigate_next() {
        let mut app = test_app();
        app.navigate_next();
        assert_eq!(app.view_month, 2);
        assert_eq!(app.view_year, 2081);
    }

    #[test]
    fn test_navigate_next_year_wrap() {
        let mut app = test_app();
        app.view_month = 12;
        app.build_view();
        app.navigate_next();
        assert_eq!(app.view_month, 1);
        assert_eq!(app.view_year, 2082);
    }

    #[test]
    fn test_navigate_prev() {
        let mut app = test_app();
        app.view_month = 3;
        app.build_view();
        app.navigate_prev();
        assert_eq!(app.view_month, 2);
    }

    #[test]
    fn test_navigate_prev_year_wrap() {
        let mut app = test_app();
        app.view_month = 1;
        app.build_view();
        app.navigate_prev();
        assert_eq!(app.view_month, 12);
        assert_eq!(app.view_year, 2080);
    }

    #[test]
    fn test_navigate_year_next() {
        let mut app = test_app();
        app.navigate_year_next();
        assert_eq!(app.view_year, 2082);
        assert_eq!(app.view_month, 1);
    }

    #[test]
    fn test_navigate_year_prev() {
        let mut app = test_app();
        app.navigate_year_prev();
        assert_eq!(app.view_year, 2080);
        assert_eq!(app.view_month, 1);
    }

    #[test]
    fn test_build_view_creates_rows() {
        let app = test_app();
        assert!(!app.calendar_rows.is_empty());
        assert!(app.calendar_rows.len() >= 4);
        assert!(app.calendar_rows.len() <= 6);
    }

    #[test]
    fn test_ad_range_str() {
        let app = test_app();
        assert!(!app.ad_range_str.is_empty());
        assert!(app.ad_range_str.contains('-'));
    }

    #[test]
    fn test_theme_selector_open_close() {
        let mut app = test_app();
        app.open_theme_selector();
        assert_eq!(app.mode, AppMode::ThemeSelector);
        app.close_theme_selector();
        assert_eq!(app.mode, AppMode::Normal);
    }

    #[test]
    fn test_theme_selector_navigation() {
        let mut app = test_app();
        app.theme_selector_selected = 0;
        app.theme_selector_next();
        assert_eq!(app.theme_selector_selected, 1);
        app.theme_selector_prev();
        assert_eq!(app.theme_selector_selected, 0);
    }

    #[test]
    fn test_theme_selector_wrap() {
        let mut app = test_app();
        let len = theme::THEME_NAMES.len();
        app.theme_selector_selected = len - 1;
        app.theme_selector_next();
        assert_eq!(app.theme_selector_selected, 0);
        app.theme_selector_prev();
        assert_eq!(app.theme_selector_selected, len - 1);
    }

    #[test]
    fn test_goto_open_close() {
        let mut app = test_app();
        app.open_goto();
        assert_eq!(app.mode, AppMode::Goto);
        assert!(app.goto_input.is_empty());
        app.close_goto();
        assert_eq!(app.mode, AppMode::Normal);
    }

    #[test]
    fn test_goto_empty_input() {
        let mut app = test_app();
        app.goto_input = "".into();
        app.apply_goto();
        assert!(app.goto_error.is_some());
    }

    #[test]
    fn test_goto_valid_year() {
        let mut app = test_app();
        app.goto_input = "2085".into();
        app.apply_goto();
        assert_eq!(app.view_year, 2085);
        assert_eq!(app.view_month, 1);
        assert_eq!(app.mode, AppMode::Normal);
    }

    #[test]
    fn test_goto_valid_year_month() {
        let mut app = test_app();
        app.goto_input = "2085-08".into();
        app.apply_goto();
        assert_eq!(app.view_year, 2085);
        assert_eq!(app.view_month, 8);
    }

    #[test]
    fn test_goto_out_of_range_year() {
        let mut app = test_app();
        app.goto_input = "3000".into();
        app.apply_goto();
        assert!(app.goto_error.is_some());
    }

    #[test]
    fn test_goto_invalid_month() {
        let mut app = test_app();
        app.goto_input = "2081-13".into();
        app.apply_goto();
        assert!(app.goto_error.is_some());
    }

    #[test]
    fn test_navigate_today() {
        let mut app = test_app();
        app.today_key = Some((2083, 5, 10));
        app.navigate_today();
        assert_eq!(app.view_year, 2083);
        assert_eq!(app.view_month, 5);
    }

    #[test]
    fn test_saturday_detection() {
        let app = test_app();
        let has_saturday =
            app.calendar_rows.iter().flat_map(|r| &r.cells).flatten().any(|c| c.is_saturday);
        assert!(has_saturday);
    }
}

#[derive(Debug, PartialEq)]
pub enum AppMode {
    Normal,
    ThemeSelector,
    Goto,
}

pub struct App {
    pub should_quit:        bool,
    pub show_small_warning: bool,
    pub mode:               AppMode,
    pub theme:              Theme,

    pub today:            Option<NepaliDate>,
    pub view_year:        i32,
    pub view_month:       u32,
    pub initial_view_set: bool,

    pub ad_range_str:  String,
    pub calendar_rows: Vec<CalendarRow>,
    pub today_key:     Option<(i32, u32, u32)>,

    pub theme_selector_selected: usize,

    pub goto_input: String,
    pub goto_error: Option<String>,
}

pub struct CalendarCell {
    pub day:         u32,
    pub is_today:    bool,
    pub is_saturday: bool,
    pub ad_day:      u32,
}

pub struct CalendarRow {
    pub cells: Vec<Option<CalendarCell>>,
}

const BS_MAX: i32 = 2099;

impl App {
    pub fn new(theme_name: &str) -> Self {
        let theme = theme::from_name(theme_name);
        let mut app = App {
            should_quit: false,
            show_small_warning: true,
            mode: AppMode::Normal,
            theme,
            today: None,
            view_year: 2081,
            view_month: 1,
            initial_view_set: false,
            ad_range_str: String::new(),
            calendar_rows: Vec::new(),
            today_key: None,
            theme_selector_selected: 0,
            goto_input: String::new(),
            goto_error: None,
        };
        app.update();
        app
    }

    pub fn update(&mut self) {
        let now = Local::now();
        self.today = calendar::ad_to_bs(now.year(), now.month(), now.day());

        if let Some(ref nd) = self.today {
            self.today_key = Some((nd.year, nd.month, nd.day));
            if !self.initial_view_set {
                self.view_year = nd.year;
                self.view_month = nd.month;
                self.initial_view_set = true;
                self.build_view();
            }
        }
    }

    pub fn navigate_next(&mut self) {
        if self.view_month >= 12 {
            self.view_month = 1;
            self.view_year += 1;
        } else {
            self.view_month += 1;
        }
        self.build_view();
        log::Log::info(&format!("Navigate next: {}/{}", self.view_year, self.view_month));
    }

    pub fn navigate_prev(&mut self) {
        if self.view_month <= 1 {
            self.view_month = 12;
            self.view_year -= 1;
        } else {
            self.view_month -= 1;
        }
        self.build_view();
        log::Log::info(&format!("Navigate prev: {}/{}", self.view_year, self.view_month));
    }

    pub fn navigate_year_next(&mut self) {
        self.view_year += 1;
        self.build_view();
        log::Log::info(&format!("Navigate year next: {}", self.view_year));
    }

    pub fn navigate_year_prev(&mut self) {
        self.view_year -= 1;
        self.build_view();
        log::Log::info(&format!("Navigate year prev: {}", self.view_year));
    }

    pub fn navigate_today(&mut self) {
        if let Some((y, m, _)) = self.today_key {
            self.view_year = y;
            self.view_month = m;
            self.build_view();
            log::Log::info("Navigate to today");
        }
    }

    pub fn open_theme_selector(&mut self) {
        self.theme_selector_selected = theme::THEME_NAMES
            .iter()
            .position(|t| theme::display_name(t) == self.theme.name)
            .unwrap_or(0);
        self.mode = AppMode::ThemeSelector;
    }

    pub fn close_theme_selector(&mut self) {
        self.mode = AppMode::Normal;
    }

    pub fn theme_selector_next(&mut self) {
        let len = theme::THEME_NAMES.len();
        self.theme_selector_selected = (self.theme_selector_selected + 1) % len;
    }

    pub fn theme_selector_prev(&mut self) {
        let len = theme::THEME_NAMES.len();
        self.theme_selector_selected = (self.theme_selector_selected + len - 1) % len;
    }

    pub fn apply_selected_theme(&mut self) {
        let name = theme::THEME_NAMES[self.theme_selector_selected];
        self.theme = theme::from_name(name);
        self.mode = AppMode::Normal;

        let mut config = Config::load();
        config.theme = Some(name.to_string());
        if let Err(e) = config.save() {
            log::Log::error(&format!("Failed to save theme config: {e}"));
        }
        log::Log::info(&format!("Theme changed to {}", name));
    }

    pub fn open_goto(&mut self) {
        self.goto_input.clear();
        self.goto_error = None;
        self.mode = AppMode::Goto;
    }

    pub fn close_goto(&mut self) {
        self.mode = AppMode::Normal;
        self.goto_error = None;
    }

    pub fn apply_goto(&mut self) {
        let input = self.goto_input.trim().to_string();
        if input.is_empty() {
            self.goto_error = Some("Enter a date (e.g. 2081, 2081-03, or 2081-03-15)".into());
            return;
        }

        let parts: Vec<&str> = input.splitn(3, '-').collect();
        let year: i32 = match parts.first().and_then(|p| p.parse().ok()) {
            Some(y) if (calendar::BS_EPOCH_YEAR..=BS_MAX).contains(&y) => y,
            _ => {
                self.goto_error = Some(format!(
                    "Year must be {}-{} (AD ~1918-2042)",
                    calendar::BS_EPOCH_YEAR,
                    BS_MAX
                ));
                return;
            }
        };

        let month: u32 = match parts.get(1) {
            Some(&m) => match m.parse::<u32>() {
                Ok(m) if (1..=12).contains(&m) => m,
                _ => {
                    self.goto_error = Some("Month must be 1-12".into());
                    return;
                }
            },
            None => 1,
        };

        if let Some(&d) = parts.get(2) {
            let day: u32 = match d.parse() {
                Ok(d) if (1..=32).contains(&d) => d,
                _ => {
                    self.goto_error = Some("Invalid day".into());
                    return;
                }
            };
            let Some(days_in_m) = get_days_in_month(year, month) else {
                self.goto_error = Some("Date out of range".into());
                return;
            };
            if day > days_in_m {
                self.goto_error = Some(format!("{}/{} has only {} days", year, month, days_in_m));
                return;
            }
        }

        self.view_year = year;
        self.view_month = month;
        self.build_view();
        self.mode = AppMode::Normal;
        self.goto_error = None;
        log::Log::info(&format!("Goto: {}/{}", year, month));
    }

    pub fn build_view(&mut self) {
        let Some(days) = get_days_in_month(self.view_year, self.view_month) else {
            return;
        };
        let Some(start_wd) = month_start_weekday(self.view_year, self.view_month) else {
            return;
        };
        let Some((ad_start, ad_end)) = month_ad_range(self.view_year, self.view_month) else {
            return;
        };

        self.ad_range_str =
            format!("{} - {}", ad_start.format("%b %d"), ad_end.format("%b %d, %Y"),);

        let mut rows: Vec<CalendarRow> = Vec::new();
        let mut current_cells: Vec<Option<CalendarCell>> = Vec::new();
        let mut cell_idx = 0usize;

        for _ in 0..start_wd {
            current_cells.push(None);
            cell_idx += 1;
        }

        for day in 1..=days {
            let ad_date = ad_start + chrono::Duration::days((day - 1) as i64);
            let is_today = self.today_key.is_some_and(|(ty, tm, td)| {
                ty == self.view_year && tm == self.view_month && td == day
            });
            let is_saturday = cell_idx % 7 == 6;
            current_cells.push(Some(CalendarCell {
                day,
                is_today,
                is_saturday,
                ad_day: ad_date.day(),
            }));
            cell_idx += 1;

            if current_cells.len() == 7 {
                rows.push(CalendarRow { cells: current_cells });
                current_cells = Vec::new();
            }
        }

        if !current_cells.is_empty() {
            while current_cells.len() < 7 {
                current_cells.push(None);
            }
            rows.push(CalendarRow { cells: current_cells });
        }

        self.calendar_rows = rows;
    }
}
