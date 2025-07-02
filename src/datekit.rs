use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
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
        (dt1.date_naive() - dt2.date_naive()).num_days().abs()
    }
}
