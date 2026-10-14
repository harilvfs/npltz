use crate::calendar;
use crate::error::{NpltzError, Result};
use calendar::NepaliDate;
use chrono::{Datelike, Local, NaiveDate};
use std::fs;
use std::io::{self, Write};

/// One compact line: `BS 2082-06-15 Aaitabar 15 · AD 2026-09-29 Monday · 12d left`.
fn format_oneline(nd: Option<&NepaliDate>, ad: &NaiveDate, days_left: Option<u32>) -> String {
    let mut out = match nd {
        Some(n) => {
            format!("BS {:04}-{:02}-{:02} {} {}", n.year, n.month, n.day, n.day_name(), n.day)
        }
        None => "BS N/A".to_string(),
    };
    out.push_str(&format!(" \u{b7} AD {}", ad.format("%Y-%m-%d %A")));
    if let Some(left) = days_left {
        out.push_str(&format!(" \u{b7} {}d left", left));
    }
    out
}

pub fn show_today(json: bool, oneline: bool) -> Result<()> {
    let now = Local::now();
    let nd = calendar::ad_to_bs(now.year(), now.month(), now.day());

    let mut out = io::stdout();

    if json {
        let obj = serde_json::json!({
            "nepali": nd.as_ref().map(|n| n.format_long()),
            "nepali_short": nd.as_ref().map(|n| format!("{:04}/{:02}/{:02}", n.year, n.month, n.day)),
            "english": now.format("%A, %B %d, %Y").to_string(),
            "english_short": now.format("%Y/%m/%d").to_string(),
            "time": now.format("%I:%M:%S %p").to_string(),
            "bs_year": nd.as_ref().map(|n| n.year),
            "bs_month": nd.as_ref().map(|n| n.month),
            "bs_day": nd.as_ref().map(|n| n.day),
            "bs_month_name": nd.as_ref().map(|n| n.month_name()),
            "bs_day_name": nd.as_ref().map(|n| n.day_name()),
            "ad_year": now.year(),
            "ad_month": now.month(),
            "ad_day": now.day(),
        });
        let _ = writeln!(out, "{}", serde_json::to_string_pretty(&obj).unwrap());
        return Ok(());
    }

    if oneline {
        let left = nd.as_ref().and_then(|n| {
            calendar::get_days_in_month(n.year, n.month).map(|t| t.saturating_sub(n.day))
        });
        let _ = writeln!(out, "{}", format_oneline(nd.as_ref(), &now.date_naive(), left));
        return Ok(());
    }

    if let Some(ref nd) = nd {
        let _ = writeln!(
            out,
            "BS : {:04}/{:02}/{:02}  ({} {}, {} {})",
            nd.year,
            nd.month,
            nd.day,
            nd.day_name(),
            nd.day,
            nd.month_name(),
            nd.year
        );
    } else {
        let _ = writeln!(out, "BS : N/A");
    }

    let _ = writeln!(
        out,
        "AD : {}  ({}, {} {}, {})",
        now.format("%Y/%m/%d"),
        now.format("%A"),
        now.format("%B"),
        now.day(),
        now.year()
    );
    let _ = writeln!(out, "Time : {}", now.format("%I:%M:%S %p"));
    Ok(())
}

pub fn show_ad_date(date_str: &str, json: bool, oneline: bool) -> Result<()> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| NpltzError::InvalidDate("Invalid date format. Use YYYY-MM-DD".into()))?;
    let nd = calendar::ad_to_bs(date.year(), date.month(), date.day());
    let mut out = io::stdout();

    if json {
        let obj = serde_json::json!({
            "nepali": nd.as_ref().map(|n| n.format_long()),
            "nepali_short": nd.as_ref().map(|n| format!("{:04}/{:02}/{:02}", n.year, n.month, n.day)),
            "bs_year": nd.as_ref().map(|n| n.year),
            "bs_month": nd.as_ref().map(|n| n.month),
            "bs_day": nd.as_ref().map(|n| n.day),
            "bs_month_name": nd.as_ref().map(|n| n.month_name()),
            "bs_day_name": nd.as_ref().map(|n| n.day_name()),
            "ad_year": date.year(),
            "ad_month": date.month(),
            "ad_day": date.day(),
        });
        let _ = writeln!(out, "{}", serde_json::to_string_pretty(&obj).unwrap());
        return Ok(());
    }

    if oneline {
        let _ = writeln!(out, "{}", format_oneline(nd.as_ref(), &date, None));
        return Ok(());
    }

    if let Some(ref nd) = nd {
        let _ = writeln!(
            out,
            "BS : {:04}/{:02}/{:02}  ({} {}, {} {})",
            nd.year,
            nd.month,
            nd.day,
            nd.day_name(),
            nd.day,
            nd.month_name(),
            nd.year
        );
    } else {
        let _ = writeln!(out, "BS : N/A");
    }

    let _ = writeln!(
        out,
        "AD : {}  ({}, {} {}, {})",
        date.format("%Y/%m/%d"),
        date.format("%A"),
        date.format("%B"),
        date.day(),
        date.year()
    );
    Ok(())
}

pub fn show_bs_date(date_str: &str, json: bool, oneline: bool) -> Result<()> {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return Err(NpltzError::InvalidDate("Invalid date format. Use YYYY-MM-DD".into()));
    }
    let year: i32 = parts[0].parse().map_err(|_| NpltzError::InvalidDate("Invalid year".into()))?;
    let month: u32 =
        parts[1].parse().map_err(|_| NpltzError::InvalidDate("Invalid month".into()))?;
    let day: u32 = parts[2].parse().map_err(|_| NpltzError::InvalidDate("Invalid day".into()))?;

    let ad = calendar::bs_to_ad(year, month, day);
    let weekday = ad.map_or(0, |d| d.format("%w").to_string().parse().unwrap_or(0));
    let nd = NepaliDate { year, month, day, weekday };
    let mut out = io::stdout();

    if json {
        let obj = serde_json::json!({
            "nepali": nd.format_long(),
            "nepali_short": format!("{:04}/{:02}/{:02}", nd.year, nd.month, nd.day),
            "english": ad.map(|d| d.format("%A, %B %d, %Y").to_string()),
            "english_short": ad.map(|d| d.format("%Y/%m/%d").to_string()),
            "bs_year": year,
            "bs_month": month,
            "bs_day": day,
            "bs_month_name": nd.month_name(),
        });
        let _ = writeln!(out, "{}", serde_json::to_string_pretty(&obj).unwrap());
        return Ok(());
    }

    if oneline {
        match ad {
            Some(ad) => {
                let _ = writeln!(out, "{}", format_oneline(Some(&nd), &ad, None));
            }
            None => {
                let _ = writeln!(out, "BS {:04}-{:02}-{:02} \u{b7} AD N/A", year, month, day);
            }
        }
        return Ok(());
    }

    let _ = writeln!(
        out,
        "BS : {:04}/{:02}/{:02}  ({} {}, {} {})",
        nd.year,
        nd.month,
        nd.day,
        nd.day_name(),
        nd.day,
        nd.month_name(),
        nd.year
    );
    if let Some(ad) = ad {
        let _ = writeln!(
            out,
            "AD : {}  ({}, {} {}, {})",
            ad.format("%Y/%m/%d"),
            ad.format("%A"),
            ad.format("%B"),
            ad.day(),
            ad.year()
        );
    } else {
        let _ = writeln!(out, "AD : N/A");
    }
    Ok(())
}

pub fn convert_ad_to_bs(date_str: &str) -> Result<()> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| NpltzError::InvalidDate("Invalid date format. Use YYYY-MM-DD".into()))?;
    let mut out = io::stdout();
    match calendar::ad_to_bs(date.year(), date.month(), date.day()) {
        Some(nd) => {
            let _ = writeln!(out, "{}", nd.format_long());
        }
        None => {
            let bs_end = calendar::BS_EPOCH_YEAR + 124;
            let _ = writeln!(
                out,
                "Date out of range. Supports AD ~{}-{} (BS {}-{})",
                1918,
                2042,
                calendar::BS_EPOCH_YEAR,
                bs_end
            );
        }
    }
    Ok(())
}

pub fn convert_bs_to_ad(date_str: &str) -> Result<()> {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return Err(NpltzError::InvalidDate("Invalid date format. Use YYYY-MM-DD".into()));
    }
    let year: i32 = parts[0].parse().map_err(|_| NpltzError::InvalidDate("Invalid year".into()))?;
    let month: u32 =
        parts[1].parse().map_err(|_| NpltzError::InvalidDate("Invalid month".into()))?;
    let day: u32 = parts[2].parse().map_err(|_| NpltzError::InvalidDate("Invalid day".into()))?;
    let mut out = io::stdout();

    match calendar::bs_to_ad(year, month, day) {
        Some(ad) => {
            let _ = writeln!(
                out,
                "{}, {} {}, {}",
                ad.format("%A"),
                ad.format("%B"),
                ad.day(),
                ad.year()
            );
        }
        None => {
            let bs_end = calendar::BS_EPOCH_YEAR + 124;
            let _ = writeln!(
                out,
                "Date out of range. Supports BS {}-{} (AD ~{}-{})",
                calendar::BS_EPOCH_YEAR,
                bs_end,
                1918,
                2042
            );
        }
    }
    Ok(())
}

pub fn show_upcoming(n: u32) -> Result<()> {
    let now = Local::now();
    let mut ad_date = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
        .ok_or_else(|| NpltzError::InvalidDate("Invalid current date".into()))?;
    let mut out = io::stdout();

    for i in 0..n {
        let nd = calendar::ad_to_bs(ad_date.year(), ad_date.month(), ad_date.day());
        if let Some(ref date) = nd {
            let _ = writeln!(
                out,
                "BS {:04}-{:02}-{:02} ({}) → AD {} ({})",
                date.year,
                date.month,
                date.day,
                date.day_name(),
                ad_date.format("%Y-%m-%d"),
                ad_date.format("%A"),
            );
        } else {
            let _ = writeln!(out, "AD {} (out of BS range)", ad_date.format("%Y-%m-%d"));
        }
        if i < n - 1 {
            ad_date += chrono::Duration::days(1);
        }
    }
    Ok(())
}

pub fn show_week() -> Result<()> {
    let now = Local::now();
    let today = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
        .ok_or_else(|| NpltzError::InvalidDate("Invalid current date".into()))?;

    let weekday = today.format("%w").to_string().parse::<i64>().unwrap_or(0);
    let sunday = today - chrono::Duration::days(weekday);

    let today_bs = calendar::ad_to_bs(now.year(), now.month(), now.day());
    let week_num = today_bs
        .as_ref()
        .map(|nd| {
            let day_of_year = calendar::get_day_of_year(nd.year, nd.month, nd.day).unwrap_or(0);
            (day_of_year - 1) / 7 + 1
        })
        .unwrap_or(0);

    let mut out = io::stdout();

    let _ = writeln!(
        out,
        "Week {} of {} ({})",
        week_num,
        today_bs.as_ref().map(|n| n.year).unwrap_or(0),
        today_bs.as_ref().map(|n| n.month_name()).unwrap_or(""),
    );
    let _ = writeln!(out);

    for i in 0..7 {
        let date = sunday + chrono::Duration::days(i);
        let nd = calendar::ad_to_bs(date.year(), date.month(), date.day());
        if let Some(ref d) = nd {
            let marker = if i == weekday { " *" } else { "" };
            let _ = writeln!(
                out,
                "{:<4} {:02}  {:<8} {:02}  → {} {}{}",
                d.day_name(),
                d.day,
                d.month_name(),
                d.month,
                date.format("%b %d"),
                date.format("%Y"),
                marker,
            );
        }
    }
    Ok(())
}

/// Print one BS month as plain ASCII. No alternate screen and no raw mode, so
/// it scrolls, pipes and copies like any other terminal output.
pub fn show_month(month: Option<&str>, show_ad: bool) -> Result<()> {
    let now = Local::now();
    let today_bs = calendar::ad_to_bs(now.year(), now.month(), now.day());
    let today = today_bs.as_ref().map(|n| (n.year, n.month, n.day));

    let (y, m) = match month {
        Some(spec) => parse_month(spec)?,
        None => match today {
            Some(t) => (t.0, t.1),
            None => {
                return Err(NpltzError::InvalidDate("Cannot determine current BS date".into()));
            }
        },
    };

    if calendar::get_days_in_month(y, m).is_none() {
        return Err(NpltzError::InvalidDate(format!("BS {y}-{m:02} is out of range")));
    }

    let mut out = io::stdout();
    let _ = writeln!(out, "{}", build_month(y, m, show_ad, today).trim_end());
    Ok(())
}

fn parse_month(spec: &str) -> Result<(i32, u32)> {
    let parts: Vec<&str> = spec.split('-').collect();
    if parts.len() != 2 {
        return Err(NpltzError::InvalidDate("Month format must be YYYY-MM".into()));
    }
    let y: i32 = parts[0].parse().map_err(|_| NpltzError::InvalidDate("Invalid year".into()))?;
    let m: u32 = parts[1].parse().map_err(|_| NpltzError::InvalidDate("Invalid month".into()))?;
    if !(1..=12).contains(&m) {
        return Err(NpltzError::InvalidDate("Month must be 1-12".into()));
    }
    Ok((y, m))
}

fn month_cell(
    day: u32,
    y: i32,
    m: u32,
    today: Option<(i32, u32, u32)>,
    w: usize,
    ad: bool,
) -> String {
    if day == 0 {
        return " ".repeat(w);
    }
    let is_today = today == Some((y, m, day));
    let value = if ad { calendar::bs_to_ad(y, m, day).map(|d| d.day()).unwrap_or(0) } else { day };
    let text = if is_today { format!("[{value}]") } else { value.to_string() };
    format!("{:^w$}", text)
}

pub(crate) fn build_month(y: i32, m: u32, show_ad: bool, today: Option<(i32, u32, u32)>) -> String {
    const W: usize = 4;

    let Some(days) = calendar::get_days_in_month(y, m) else {
        return format!("BS {y}-{m:02} is out of range");
    };
    let start_wd = calendar::month_start_weekday(y, m).unwrap_or(0);
    let ad_range = calendar::month_ad_range(y, m);

    let mut s = String::new();
    s.push_str(&format!("{} {}\n", calendar::english_month_name(m), y));
    if let Some((start, end)) = ad_range {
        s.push_str(&format!("{} - {}\n", start.format("%b %d"), end.format("%b %d, %Y")));
    }
    s.push('\n');

    let header: String = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]
        .iter()
        .map(|h| format!("{:^W$}", h))
        .collect();
    s.push_str(header.trim_end());
    s.push('\n');

    let mut cells: Vec<u32> = vec![0; start_wd];
    cells.extend(1..=days);
    while !cells.len().is_multiple_of(7) {
        cells.push(0);
    }

    for week in cells.chunks(7) {
        let line: String = week.iter().map(|d| month_cell(*d, y, m, today, W, false)).collect();
        s.push_str(line.trim_end());
        s.push('\n');

        if show_ad {
            let ad_line: String =
                week.iter().map(|d| month_cell(*d, y, m, today, W, true)).collect();
            s.push_str(ad_line.trim_end());
            s.push('\n');
        }
    }

    if let Some(t) = today
        && t.0 == y
        && t.1 == m
    {
        s.push_str(&format!("\n[{}] = today\n", t.2));
    }

    s
}

pub fn export_ics(month: Option<&str>, count: Option<u32>, output: Option<&str>) -> Result<()> {
    let now = Local::now();
    let today_bs = calendar::ad_to_bs(now.year(), now.month(), now.day())
        .ok_or_else(|| NpltzError::InvalidDate("Cannot determine current BS date".into()))?;

    let (start_year, start_month) = if let Some(m) = month {
        let parts: Vec<&str> = m.split('-').collect();
        if parts.len() != 2 {
            return Err(NpltzError::InvalidDate("Month format must be YYYY-MM".into()));
        }
        let y: i32 =
            parts[0].parse().map_err(|_| NpltzError::InvalidDate("Invalid year".into()))?;
        let mo: u32 =
            parts[1].parse().map_err(|_| NpltzError::InvalidDate("Invalid month".into()))?;
        if !(1..=12).contains(&mo) {
            return Err(NpltzError::InvalidDate("Month must be 1-12".into()));
        }
        (y, mo)
    } else {
        (today_bs.year, today_bs.month)
    };

    let num_months = count.unwrap_or(1);

    let mut lines: Vec<String> = vec![
        "BEGIN:VCALENDAR".into(),
        "VERSION:2.0".into(),
        "PRODID:-//npltz//Nepali Calendar//EN".into(),
        "CALSCALE:GREGORIAN".into(),
        "METHOD:PUBLISH".into(),
        "X-WR-CALNAME:Nepali Calendar (BS)".into(),
    ];

    for offset in 0..num_months {
        let m = ((start_month - 1 + offset) % 12) + 1;
        let y = start_year + ((start_month - 1 + offset) / 12) as i32;

        let days_in_month = match calendar::get_days_in_month(y, m) {
            Some(d) => d,
            None => continue,
        };

        for day in 1..=days_in_month {
            let ad = match calendar::bs_to_ad(y, m, day) {
                Some(d) => d,
                None => continue,
            };

            let nd = NepaliDate {
                year: y,
                month: m,
                day,
                weekday: ad.format("%w").to_string().parse().unwrap_or(0),
            };

            let ad_str = ad.format("%Y%m%d").to_string();
            let next_day = ad + chrono::Duration::days(1);
            let ad_end = next_day.format("%Y%m%d").to_string();

            let uid = format!("npltz-{:04}{:02}{:02}@npltz", y, m, day);
            let summary =
                format!("BS {:04}-{:02}-{:02} ({} {})", y, m, day, nd.day_name(), nd.month_name(),);
            let description = format!(
                "Bikram Sambat: {} {}\nGregorian: {} {}",
                nd.day_name(),
                nd.month_name(),
                ad.format("%A"),
                ad.format("%B %d, %Y"),
            );

            lines.push("BEGIN:VEVENT".into());
            lines.push(format!("DTSTART;VALUE=DATE:{ad_str}"));
            lines.push(format!("DTEND;VALUE=DATE:{ad_end}"));
            lines.push(format!("DTSTAMP:{}", now.format("%Y%m%dT%H%M%SZ")));
            lines.push(format!("UID:{uid}"));
            lines.push(format!("SUMMARY:{summary}"));
            lines.push(format!("DESCRIPTION:{description}"));
            lines.push("TRANSP:TRANSPARENT".into());
            lines.push("END:VEVENT".into());
        }
    }

    lines.push("END:VCALENDAR".into());

    let ics_content = lines.join("\r\n");
    let out_path = output.unwrap_or("nepali_calendar.ics");

    fs::write(out_path, &ics_content)?;

    let mut out = io::stdout();
    let _ = writeln!(out, "Exported BS calendar to {out_path}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_oneline_has_both_calendars() {
        let nd = calendar::ad_to_bs(2026, 9, 29).unwrap();
        let ad = NaiveDate::from_ymd_opt(2026, 9, 29).unwrap();
        let line = format_oneline(Some(&nd), &ad, Some(12));

        assert!(line.contains(&format!("BS {:04}-{:02}-{:02}", nd.year, nd.month, nd.day)));
        assert!(line.contains(nd.day_name()));
        assert!(line.contains("AD 2026-09-29"));
        assert!(line.contains("12d left"));
        assert_eq!(line.matches('\u{b7}').count(), 2);
    }

    #[test]
    fn test_format_oneline_without_nepali_date() {
        let ad = NaiveDate::from_ymd_opt(2026, 9, 29).unwrap();
        let line = format_oneline(None, &ad, None);
        assert!(line.starts_with("BS N/A"));
        assert!(!line.contains("left"));
    }

    #[test]
    fn test_parse_month() {
        assert_eq!(parse_month("2081-03").unwrap(), (2081, 3));
        assert!(parse_month("2081").is_err());
        assert!(parse_month("2081-13").is_err());
        assert!(parse_month("2081-0").is_err());
        assert!(parse_month("abc-03").is_err());
        assert!(parse_month("2081-03-15").is_err());
    }

    #[test]
    fn test_build_month_layout() {
        let text = build_month(2081, 1, false, None);
        let lines: Vec<&str> = text.lines().collect();

        assert!(lines[0].contains("Baisakh 2081"), "title: {}", lines[0]);
        assert!(lines[1].contains(" - "), "ad range: {}", lines[1]);
        assert_eq!(lines[2], "");
        assert!(lines[3].starts_with("Sun"), "header: {}", lines[3]);

        let weeks = lines.len() - 4;
        assert!((5..=6).contains(&weeks), "weeks: {weeks}");

        // Every week line stays within 7 cells of 4 columns.
        for line in &lines[4..] {
            assert!(line.chars().count() <= 28, "line too wide: '{line}'");
        }
    }

    #[test]
    fn test_build_month_marks_today() {
        let text = build_month(2081, 1, false, Some((2081, 1, 15)));
        assert!(text.contains("[15]"));
        assert!(text.contains("[15] = today"));
    }

    #[test]
    fn test_build_month_skips_marker_for_other_month() {
        let text = build_month(2081, 1, false, Some((2081, 2, 15)));
        assert!(!text.contains("= today"));
    }

    #[test]
    fn test_build_month_ad_rows_add_one_line_per_week() {
        let plain_text = build_month(2081, 1, false, None);
        let ad_text = build_month(2081, 1, true, None);
        let plain: Vec<&str> = plain_text.lines().collect();
        let with_ad: Vec<&str> = ad_text.lines().collect();
        let weeks = plain.len() - 4;
        assert_eq!(with_ad.len(), plain.len() + weeks);
    }

    #[test]
    fn test_build_month_out_of_range() {
        let text = build_month(3000, 1, false, None);
        assert!(text.contains("out of range"));
    }
}
