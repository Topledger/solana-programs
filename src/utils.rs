pub fn convert_to_date(timestamp: i64) -> String {
    let seconds_per_day: i64 = 24 * 60 * 60;
    let unix_epoch: i64 = 1609459200; // Unix epoch timestamp (January 1, 2021)
    
    let total_days = (timestamp - unix_epoch) / seconds_per_day;
    
    let year = 2021 + (total_days / 365);
    let mut remaining_days = total_days % 365;
    
    let mut is_leap_year = false;
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        is_leap_year = true;
    }
    
    if is_leap_year && remaining_days >= 59 {
        remaining_days -= 1;
    }
    
    let months = [
        31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31
    ];
    
    let mut month = 0;
    for days_in_month in &months {
        if remaining_days < *days_in_month {
            break;
        }
        remaining_days -= days_in_month;
        month += 1;
    }
    
    let day = remaining_days + 1;
    
    format!("{:04}-{:02}-{:02}", year, month + 1, day)
}