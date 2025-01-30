use chrono::{DateTime, Datelike, Local};

pub fn is_same_day(smoked_date: &DateTime<Local>, date_now: &DateTime<Local>) -> bool {
    smoked_date.date_naive() == date_now.date_naive()
}

pub fn is_same_week(smoked_date: &DateTime<Local>, date_now: &DateTime<Local>) -> bool {
    smoked_date.iso_week() == date_now.iso_week()
}

pub fn is_same_month(smoked_date: &DateTime<Local>, date_now: &DateTime<Local>) -> bool {
    smoked_date.month() == date_now.month() && smoked_date.year() == date_now.year()
}

pub fn is_same_year(smoked_date: &DateTime<Local>, date_now: &DateTime<Local>) -> bool {
    smoked_date.year() == date_now.year()
}
