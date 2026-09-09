use chrono::NaiveDate;

pub mod app;
pub mod controllers;
pub mod mailers;
pub mod models;
pub mod tasks;
pub mod views;

pub const PIVOT: u32 = 26;
pub fn parse_birthdate(birthdate: &str) -> Option<NaiveDate> {
    let parts: Vec<&str> = birthdate.split(".").collect();
    if parts.len() != 3 {
        return None;
    }
    let day: u32 = parts[0].parse().ok()?;
    let month = parts[1].parse().ok()?;
    let year_str = parts[2];
    let year: u32 = year_str.parse().ok()?;
    let full_year = if year_str.len() == 4 {
        year
    } else {
        if year <= PIVOT {
            year + 2000
        } else {
            year + 1900
        }
    };

    NaiveDate::from_ymd_opt(full_year as i32, month, day)
}
