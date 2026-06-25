use chrono::NaiveDate;
use serde::Deserialize;

const MONTH_NAMES: &[&str] = &[
    "Baisakh", "Jestha", "Ashad", "Shrawan", "Bhadra", "Ashwin", "Kartik", "Mangsir", "Poush",
    "Magh", "Falgun", "Chaitra",
];

const DAY_NAMES: &[&str] =
    &["Aaitabar", "Sombar", "Mangalbar", "Budhabar", "Bihibar", "Shukrabar", "Shanibar"];

pub struct NepaliDate {
    pub year:    i32,
    pub month:   u32,
    pub day:     u32,
    pub weekday: usize,
}

impl NepaliDate {
    pub fn month_name(&self) -> &'static str {
        MONTH_NAMES[self.month as usize - 1]
    }

    pub fn day_name(&self) -> &'static str {
        DAY_NAMES[self.weekday]
    }

    pub fn format_long(&self) -> String {
        format!("{} {}, {} {}", self.day_name(), self.day, self.month_name(), self.year)
    }
}

#[derive(Deserialize)]
struct YearData {
    year:       i32,
    #[serde(deserialize_with = "parse_months")]
    months:     [u32; 12],
    total_days: u32,
}

fn parse_months<'de, D>(deserializer: D) -> Result<[u32; 12], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v: Vec<u32> = serde::Deserialize::deserialize(deserializer)?;
    let mut arr = [0u32; 12];
    for (i, val) in v.iter().take(12).enumerate() {
        arr[i] = *val;
    }
    Ok(arr)
}

static DATA: std::sync::LazyLock<Vec<YearData>> = std::sync::LazyLock::new(|| {
    let json_str = include_str!("../calendar_data.json");
    serde_json::from_str::<Vec<YearData>>(json_str).expect("calendar_data.json is valid")
});

const AD_EPOCH: (i32, u32, u32) = (1918, 4, 12);
pub const BS_EPOCH_YEAR: i32 = 1975;

fn get_year_data(bs_year: i32) -> Option<&'static YearData> {
    DATA.iter().find(|yd| yd.year == bs_year)
}

pub fn ad_to_bs(ad_year: i32, ad_month: u32, ad_day: u32) -> Option<NepaliDate> {
    let ad_date = NaiveDate::from_ymd_opt(ad_year, ad_month, ad_day)?;
    let weekday = ad_date.format("%w").to_string().parse().unwrap_or(0) as usize;

    let epoch_ad = NaiveDate::from_ymd_opt(AD_EPOCH.0, AD_EPOCH.1, AD_EPOCH.2)?;
    let total_days = (ad_date - epoch_ad).num_days();
    if total_days < 0 {
        return None;
    }

    let mut remaining = total_days;
    let mut year = BS_EPOCH_YEAR;

    loop {
        let yd = get_year_data(year)?;
        if remaining < yd.total_days as i64 {
            for (month, &days_in_m) in yd.months.iter().enumerate() {
                if remaining < days_in_m as i64 {
                    return Some(NepaliDate {
                        year,
                        month: month as u32 + 1,
                        day: (remaining + 1) as u32,
                        weekday,
                    });
                }
                remaining -= days_in_m as i64;
            }
            return None;
        }
        remaining -= yd.total_days as i64;
        year += 1;
    }
}

pub fn bs_to_ad(bs_year: i32, bs_month: u32, bs_day: u32) -> Option<NaiveDate> {
    let epoch_ad = NaiveDate::from_ymd_opt(AD_EPOCH.0, AD_EPOCH.1, AD_EPOCH.2)?;
    let mut total_days: i64 = 0;
    let mut year = BS_EPOCH_YEAR;

    while year < bs_year {
        let yd = get_year_data(year)?;
        total_days += yd.total_days as i64;
        year += 1;
    }

    let yd = get_year_data(bs_year)?;
    for m in 0..(bs_month as usize - 1) {
        total_days += yd.months[m] as i64;
    }
    total_days += (bs_day - 1) as i64;

    epoch_ad.checked_add_signed(chrono::Duration::days(total_days))
}

pub fn get_days_in_month(bs_year: i32, bs_month: u32) -> Option<u32> {
    let yd = get_year_data(bs_year)?;
    Some(yd.months[bs_month as usize - 1])
}

pub fn month_start_weekday(bs_year: i32, bs_month: u32) -> Option<usize> {
    let ad = bs_to_ad(bs_year, bs_month, 1)?;
    Some(ad.format("%w").to_string().parse().unwrap_or(0))
}

pub fn month_ad_range(bs_year: i32, bs_month: u32) -> Option<(NaiveDate, NaiveDate)> {
    let days = get_days_in_month(bs_year, bs_month)?;
    let start = bs_to_ad(bs_year, bs_month, 1)?;
    let end = bs_to_ad(bs_year, bs_month, days)?;
    Some((start, end))
}

pub fn english_month_name(month: u32) -> &'static str {
    MONTH_NAMES[month as usize - 1]
}

pub fn get_year_total_days(bs_year: i32) -> Option<u32> {
    let yd = get_year_data(bs_year)?;
    Some(yd.total_days)
}

pub fn get_day_of_year(bs_year: i32, bs_month: u32, bs_day: u32) -> Option<u32> {
    let yd = get_year_data(bs_year)?;
    let mut total = 0u32;
    for m in 0..(bs_month as usize - 1) {
        total += yd.months[m];
    }
    total += bs_day;
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_reference_date() {
        let nd = ad_to_bs(2024, 4, 13).unwrap();
        assert_eq!(nd.year, 2081);
        assert_eq!(nd.month, 1);
        assert_eq!(nd.day, 1);
        assert_eq!(nd.weekday, 6);
    }

    #[test]
    fn test_known_date() {
        let nd = ad_to_bs(2024, 4, 12).unwrap();
        assert_eq!(nd.year, 2080);
        assert_eq!(nd.month, 12);
        assert_eq!(nd.day, 31);
    }

    #[test]
    fn test_today() {
        let nd = ad_to_bs(2026, 6, 20).unwrap();
        assert_eq!(nd.year, 2083);
        assert_eq!(nd.month, 3);
        assert_eq!(nd.day, 6);
        assert_eq!(nd.weekday, 6);
    }

    #[test]
    fn test_days_in_month() {
        let days = get_days_in_month(2081, 1).unwrap();
        assert!(days == 30 || days == 31 || days == 32);
    }

    #[test]
    fn test_month_range() {
        let (start, end) = month_ad_range(2081, 1).unwrap();
        assert!(end >= start);
    }

    #[test]
    fn test_weekday_range() {
        let wd = month_start_weekday(2081, 1).unwrap();
        assert!(wd < 7);
    }

    #[test]
    fn test_roundtrip() {
        for (y, m, d) in &[(2024, 6, 1), (2025, 1, 15), (2026, 6, 20)] {
            let nd = ad_to_bs(*y, *m, *d).unwrap();
            let ad = bs_to_ad(nd.year, nd.month, nd.day).unwrap();
            assert_eq!((ad.year(), ad.month(), ad.day()), (*y, *m, *d));
        }
    }
}
