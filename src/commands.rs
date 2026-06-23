use crate::calendar;
use crate::error::{NpltzError, Result};
use calendar::NepaliDate;
use chrono::{Datelike, Local, NaiveDate};

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
            "Nepali  : {:04}/{:02}/{:02}  ({} {}, {} {})",
            nd.year,
            nd.month,
            nd.day,
            nd.day_name(),
            nd.day,
            nd.month_name(),
            nd.year
        );
    } else {
        println!("Nepali  : N/A");
    }

    println!(
        "English : {}  ({}, {} {}, {})",
        now.format("%Y/%m/%d"),
        now.format("%A"),
        now.format("%B"),
        now.day(),
        now.year()
    );
    println!("Time    : {}", now.format("%I:%M:%S %p"));
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
            "Nepali  : {:04}/{:02}/{:02}  ({} {}, {} {})",
            nd.year,
            nd.month,
            nd.day,
            nd.day_name(),
            nd.day,
            nd.month_name(),
            nd.year
        );
    } else {
        println!("Nepali  : N/A");
    }

    println!(
        "English : {}  ({}, {} {}, {})",
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
        "Nepali  : {:04}/{:02}/{:02}  ({} {}, {} {})",
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
            "English : {}  ({}, {} {}, {})",
            ad.format("%Y/%m/%d"),
            ad.format("%A"),
            ad.format("%B"),
            ad.day(),
            ad.year()
        );
    } else {
        println!("English : N/A");
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
