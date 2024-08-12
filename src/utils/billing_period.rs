use chrono::{Datelike, NaiveDate, Utc};

pub struct BillingPeriod {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl BillingPeriod {
    pub fn get() -> Self {
        let today = Utc::now().naive_local().date();
        let first_this_month = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
        let first_last_month = if today.month() == 1 {
            NaiveDate::from_ymd_opt(today.year() - 1, 12, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(today.year(), today.month0(), 1).unwrap()
        };

        Self {
            start_date: first_last_month,
            end_date: first_this_month,
        }
    }
}
