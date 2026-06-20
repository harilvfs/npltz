use crate::battery::BatteryInfo;
use crate::calendar::{
    self, NepaliDate, english_month_name, get_days_in_month, month_ad_range, month_start_weekday,
};
use crate::theme::Theme;
use chrono::{Datelike, Local};

pub struct App {
    pub should_quit: bool,
    pub battery: BatteryInfo,
    pub theme: Theme,

    pub today: Option<NepaliDate>,
    pub view_year: i32,
    pub view_month: u32,
    pub initial_view_set: bool,

    pub ad_range_str: String,
    pub calendar_rows: Vec<CalendarRow>,
    pub today_key: Option<(i32, u32, u32)>,
}

pub struct CalendarCell {
    pub day: u32,
    pub is_today: bool,
}

pub struct CalendarRow {
    pub cells: Vec<Option<CalendarCell>>,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            should_quit: false,
            battery: BatteryInfo::default(),
            theme: Theme::default(),
            today: None,
            view_year: 2081,
            view_month: 1,
            initial_view_set: false,
            ad_range_str: String::new(),
            calendar_rows: Vec::new(),
            today_key: None,
        };
        app.update();
        app
    }

    pub fn update(&mut self) {
        let now = Local::now();
        self.battery = crate::battery::get_battery_info();
        self.today = calendar::ad_to_bs(now.year(), now.month(), now.day());

        if let Some(ref nd) = self.today {
            self.today_key = Some((nd.year, nd.month, nd.day));
            if !self.initial_view_set {
                self.view_year = nd.year;
                self.view_month = nd.month;
                self.initial_view_set = true;
            }
        }

        self.build_view();
    }

    pub fn navigate_next(&mut self) {
        if self.view_month >= 12 {
            self.view_month = 1;
            self.view_year += 1;
        } else {
            self.view_month += 1;
        }
        self.build_view();
    }

    pub fn navigate_prev(&mut self) {
        if self.view_month <= 1 {
            self.view_month = 12;
            self.view_year -= 1;
        } else {
            self.view_month -= 1;
        }
        self.build_view();
    }

    pub fn navigate_today(&mut self) {
        if let Some((y, m, _)) = self.today_key {
            self.view_year = y;
            self.view_month = m;
            self.build_view();
        }
    }

    fn build_view(&mut self) {
        let Some(days) = get_days_in_month(self.view_year, self.view_month) else {
            return;
        };
        let Some(start_wd) = month_start_weekday(self.view_year, self.view_month) else {
            return;
        };
        let Some((ad_start, ad_end)) = month_ad_range(self.view_year, self.view_month) else {
            return;
        };

        self.ad_range_str = format!(
            "{} {} - {}",
            english_month_name(self.view_month),
            ad_start.format("%d"),
            ad_end.format("%B %d, %Y"),
        );

        let mut rows: Vec<CalendarRow> = Vec::new();
        let mut current_cells: Vec<Option<CalendarCell>> = Vec::new();

        for _ in 0..start_wd {
            current_cells.push(None);
        }

        for day in 1..=days {
            let is_today = self.today_key.is_some_and(|(ty, tm, td)| {
                ty == self.view_year && tm == self.view_month && td == day
            });
            current_cells.push(Some(CalendarCell { day, is_today }));

            if current_cells.len() == 7 {
                rows.push(CalendarRow {
                    cells: current_cells,
                });
                current_cells = Vec::new();
            }
        }

        if !current_cells.is_empty() {
            while current_cells.len() < 7 {
                current_cells.push(None);
            }
            rows.push(CalendarRow {
                cells: current_cells,
            });
        }

        self.calendar_rows = rows;
    }
}
