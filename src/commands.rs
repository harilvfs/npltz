use crate::calendar;
use crate::error::{NpltzError, Result};
use calendar::NepaliDate;
use chrono::{Datelike, Local, NaiveDate};
use std::fs;

pub fn show_today(json: bool) -> Result<()> {
    let now = Local::now();
    let nd = calendar::ad_to_bs(now.year(), now.month(), now.day());

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
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        return Ok(());
    }

    if let Some(ref nd) = nd {
        println!(
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
        println!("BS : N/A");
    }

    println!(
        "AD : {}  ({}, {} {}, {})",
        now.format("%Y/%m/%d"),
        now.format("%A"),
        now.format("%B"),
        now.day(),
        now.year()
    );
    println!("Time : {}", now.format("%I:%M:%S %p"));
    Ok(())
}

pub fn show_ad_date(date_str: &str, json: bool) -> Result<()> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| NpltzError::InvalidDate("Invalid date format. Use YYYY-MM-DD".into()))?;
    let nd = calendar::ad_to_bs(date.year(), date.month(), date.day());

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
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        return Ok(());
    }

    if let Some(ref nd) = nd {
        println!(
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
        println!("BS : N/A");
    }

    println!(
        "AD : {}  ({}, {} {}, {})",
        date.format("%Y/%m/%d"),
        date.format("%A"),
        date.format("%B"),
        date.day(),
        date.year()
    );
    Ok(())
}

pub fn show_bs_date(date_str: &str, json: bool) -> Result<()> {
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
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        return Ok(());
    }

    println!(
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
        println!(
            "AD : {}  ({}, {} {}, {})",
            ad.format("%Y/%m/%d"),
            ad.format("%A"),
            ad.format("%B"),
            ad.day(),
            ad.year()
        );
    } else {
        println!("AD : N/A");
    }
    Ok(())
}

pub fn convert_ad_to_bs(date_str: &str) -> Result<()> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| NpltzError::InvalidDate("Invalid date format. Use YYYY-MM-DD".into()))?;
    match calendar::ad_to_bs(date.year(), date.month(), date.day()) {
        Some(nd) => {
            println!("{}", nd.format_long());
        }
        None => {
            let bs_end = calendar::BS_EPOCH_YEAR + 124;
            println!(
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

    match calendar::bs_to_ad(year, month, day) {
        Some(ad) => {
            println!("{}, {} {}, {}", ad.format("%A"), ad.format("%B"), ad.day(), ad.year());
        }
        None => {
            let bs_end = calendar::BS_EPOCH_YEAR + 124;
            println!(
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

    for i in 0..n {
        let nd = calendar::ad_to_bs(ad_date.year(), ad_date.month(), ad_date.day());
        if let Some(ref date) = nd {
            println!(
                "BS {:04}-{:02}-{:02} ({}) → AD {} ({})",
                date.year,
                date.month,
                date.day,
                date.day_name(),
                ad_date.format("%Y-%m-%d"),
                ad_date.format("%A"),
            );
        } else {
            println!("AD {} (out of BS range)", ad_date.format("%Y-%m-%d"));
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

    println!(
        "Week {} of {} ({})",
        week_num,
        today_bs.as_ref().map(|n| n.year).unwrap_or(0),
        today_bs.as_ref().map(|n| n.month_name()).unwrap_or(""),
    );
    println!();

    for i in 0..7 {
        let date = sunday + chrono::Duration::days(i);
        let nd = calendar::ad_to_bs(date.year(), date.month(), date.day());
        if let Some(ref d) = nd {
            let marker = if i == weekday { " *" } else { "" };
            println!(
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

    println!("Exported BS calendar to {out_path}");
    Ok(())
}
