use chrono::{DateTime, Duration, Local, NaiveDateTime, TimeZone};
use std::time::{SystemTime, UNIX_EPOCH};

/// 获取当前时间的 DateTime<Local>
pub fn now() -> DateTime<Local> {
    Local::now()
}

/// 格式化时间为字符串，格式示例："2006-01-02 15:04:05"
pub fn format(dt: DateTime<Local>, fmt: &str) -> String {
    dt.format(fmt).to_string()
}

/// 解析时间字符串为 DateTime<Local>
/// 支持的格式可以传入如 "%Y-%m-%d %H:%M:%S"
pub fn parse(s: &str, fmt: &str) -> Result<DateTime<Local>, chrono::ParseError> {
    let ndt = NaiveDateTime::parse_from_str(s, fmt)?;
    Ok(Local.from_local_datetime(&ndt).unwrap())
}

/// 时间戳转 DateTime<Local>
pub fn timestamp_to_datetime(ts: i64) -> DateTime<Local> {
    Local.timestamp_opt(ts, 0).unwrap()
}

/// DateTime<Local> 转时间戳（秒）
pub fn datetime_to_timestamp(dt: DateTime<Local>) -> i64 {
    dt.timestamp()
}

/// 计算两个时间的差，返回秒数
pub fn diff_in_seconds(dt1: DateTime<Local>, dt2: DateTime<Local>) -> i64 {
    (dt1 - dt2).num_seconds()
}

/// 获取系统当前时间戳（秒）
pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}

/// 判断是否闰年
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// 获取某月天数
pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// 时间加减相关操作，返回新的 DateTime<Local>
pub mod arithmetic {
    use super::*;
    use chrono::Duration;

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
        let dt1 = Local.ymd(2025, 6, 20).and_hms(12, 0, 0);
        let dt2 = Local.ymd(2025, 6, 20).and_hms(11, 0, 0);
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

    /// 增加天数
    pub fn add_days(dt: DateTime<Local>, days: i64) -> DateTime<Local> {
        dt + Duration::days(days)
    }

    /// 减少天数
    pub fn sub_days(dt: DateTime<Local>, days: i64) -> DateTime<Local> {
        dt - Duration::days(days)
    }

    /// 增加小时数
    pub fn add_hours(dt: DateTime<Local>, hours: i64) -> DateTime<Local> {
        dt + Duration::hours(hours)
    }

    /// 减少小时数
    pub fn sub_hours(dt: DateTime<Local>, hours: i64) -> DateTime<Local> {
        dt - Duration::hours(hours)
    }

    /// 增加分钟数
    pub fn add_minutes(dt: DateTime<Local>, minutes: i64) -> DateTime<Local> {
        dt + Duration::minutes(minutes)
    }

    /// 减少分钟数
    pub fn sub_minutes(dt: DateTime<Local>, minutes: i64) -> DateTime<Local> {
        dt - Duration::minutes(minutes)
    }

    /// 计算两个时间之间的天数差（绝对值）
    pub fn diff_in_days(dt1: DateTime<Local>, dt2: DateTime<Local>) -> i64 {
        (dt1.date() - dt2.date()).num_days().abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Timelike};

    #[test]
    fn test_add_sub_days() {
        let dt = Local.ymd(2025, 6, 20).and_hms(12, 0, 0);
        let dt2 = arithmetic::add_days(dt, 5);
        assert_eq!(dt2.date(), Local.ymd(2025, 6, 25));
        let dt3 = arithmetic::sub_days(dt2, 5);
        assert_eq!(dt3.date(), dt.date());
    }

    #[test]
    fn test_add_sub_hours() {
        let dt = Local.ymd(2025, 6, 20).and_hms(12, 0, 0);
        let dt2 = arithmetic::add_hours(dt, 10);
        assert_eq!(dt2.hour(), 22);
        let dt3 = arithmetic::sub_hours(dt2, 10);
        assert_eq!(dt3.hour(), dt.hour());
    }

    #[test]
    fn test_add_sub_minutes() {
        let dt = Local.ymd(2025, 6, 20).and_hms(12, 0, 0);
        let dt2 = arithmetic::add_minutes(dt, 30);
        assert_eq!(dt2.minute(), 30);
        let dt3 = arithmetic::sub_minutes(dt2, 30);
        assert_eq!(dt3.minute(), dt.minute());
    }

    #[test]
    fn test_diff_in_days() {
        let dt1 = Local.ymd(2025, 6, 25).and_hms(0, 0, 0);
        let dt2 = Local.ymd(2025, 6, 20).and_hms(23, 59, 59);
        assert_eq!(arithmetic::diff_in_days(dt1, dt2), 5);
    }
}
