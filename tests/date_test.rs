#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone, Timelike};
    use rovkit::datekit::{
        arithmetic, current_timestamp, datetime_to_timestamp, days_in_month, diff_in_seconds,
        format, is_leap_year, now, parse, timestamp_to_datetime,
    };

    #[test]
    fn test_now_format_parse() {
        let now = now();
        let fmt = "%Y-%m-%d %H:%M:%S";
        let s = format(now, fmt);
        let parsed = parse(&s, fmt).unwrap();
        assert_eq!(format(parsed, fmt), s);
    }

    #[test]
    fn test_timestamp_conversion() {
        let ts = current_timestamp();
        let dt = timestamp_to_datetime(ts);
        let ts2 = datetime_to_timestamp(dt);
        assert_eq!(ts, ts2);
    }

    #[test]
    fn test_diff_in_seconds() {
        let dt1 = Local.with_ymd_and_hms(2025, 6, 20, 12, 0, 0).unwrap();
        let dt2 = Local.with_ymd_and_hms(2025, 6, 20, 11, 0, 0).unwrap();
        assert_eq!(diff_in_seconds(dt1, dt2), 3600);
    }

    #[test]
    fn test_leap_year_and_days() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2025));
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2025, 2), 28);
        assert_eq!(days_in_month(2025, 4), 30);
        assert_eq!(days_in_month(2025, 5), 31);
    }
    #[test]
    fn test_add_sub_days() {
        let dt = Local.with_ymd_and_hms(2025, 6, 20, 12, 0, 0).unwrap();
        let dt2 = arithmetic::add_days(dt, 5);
        let d3 = Local.with_ymd_and_hms(2025, 6, 25, 0, 0, 0).unwrap();
        assert_eq!(dt2.date_naive(), d3.date_naive());
        let dt3 = arithmetic::sub_days(dt2, 5);
        assert_eq!(dt3.date_naive(), dt.date_naive());
    }

    #[test]
    fn test_add_sub_hours() {
        let dt = Local.with_ymd_and_hms(2025, 6, 20, 12, 0, 0).unwrap();
        let dt2 = arithmetic::add_hours(dt, 10);
        assert_eq!(dt2.hour(), 22);
        let dt3 = arithmetic::sub_hours(dt2, 10);
        assert_eq!(dt3.hour(), dt.hour());
    }

    #[test]
    fn test_add_sub_minutes() {
        let dt = Local.with_ymd_and_hms(2025, 6, 20, 12, 0, 0).unwrap();
        let dt2 = arithmetic::add_minutes(dt, 30);
        assert_eq!(dt2.minute(), 30);
        let dt3 = arithmetic::sub_minutes(dt2, 30);
        assert_eq!(dt3.minute(), dt.minute());
    }

    #[test]
    fn test_diff_in_days() {
        let dt1 = Local.with_ymd_and_hms(2025, 6, 25, 0, 0, 0).unwrap();
        let dt2 = Local.with_ymd_and_hms(2025, 6, 20, 23, 59, 59).unwrap();
        assert_eq!(arithmetic::diff_in_days(dt1, dt2), 5);
    }
}
