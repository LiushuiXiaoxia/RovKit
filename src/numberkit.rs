use std::fmt::Display;
use std::num::ParseFloatError;
use std::num::ParseIntError;
use std::str::FromStr;

/// 判断是否为整数（即小数部分为 0）
pub fn is_integer(n: f64) -> bool {
    n.fract() == 0.0
}

/// 判断是否为小数（即小数部分不为 0）
pub fn is_float(n: f64) -> bool {
    n.fract() != 0.0
}

/// 判断是否为正数
pub fn is_positive(n: f64) -> bool {
    n > 0.0
}

/// 判断是否为负数
pub fn is_negative(n: f64) -> bool {
    n < 0.0
}

/// 判断是否为零（容忍误差）
pub fn is_zero(n: f64) -> bool {
    n.abs() < std::f64::EPSILON
}

/// 判断是否为偶数
pub fn is_even(n: i64) -> bool {
    n % 2 == 0
}

/// 判断是否为奇数
pub fn is_odd(n: i64) -> bool {
    n % 2 != 0
}

/// 四舍五入到指定位数
pub fn round(n: f64, digits: u32) -> f64 {
    let factor = 10f64.powi(digits as i32);
    (n * factor).round() / factor
}

/// 向上取整
pub fn ceil(n: f64) -> f64 {
    n.ceil()
}

/// 向下取整
pub fn floor(n: f64) -> f64 {
    n.floor()
}

/// 绝对值
pub fn abs(n: f64) -> f64 {
    n.abs()
}

/// 格式化为小数（指定位数）
pub fn format_float(n: f64, digits: usize) -> String {
    format!("{:.1$}", n, digits)
}

/// 格式化为百分比字符串
pub fn format_percent(n: f64, digits: usize) -> String {
    format!("{:.1$}%", n * 100.0, digits)
}

/// 格式化带千位分隔符的数字（浮点）
pub fn format_thousands(n: f64) -> String {
    let s = format!("{:.2}", n);
    let mut parts = s.split('.');
    let int_part = parts.next().unwrap_or("");
    let dec_part = parts.next().unwrap_or("");
    let int_formatted = int_part
        .chars()
        .rev()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(",");
    let int_formatted = int_formatted.chars().rev().collect::<String>();
    format!("{}.{}", int_formatted, dec_part)
}

/// 字符串转 f64
pub fn parse_f64(s: &str) -> Result<f64, ParseFloatError> {
    f64::from_str(s.trim())
}

/// 字符串转 i64
pub fn parse_i64(s: &str) -> Result<i64, ParseIntError> {
    i64::from_str(s.trim())
}

/// 任意可 Display 的类型转字符串
pub fn to_string<T: Display>(value: T) -> String {
    value.to_string()
}

/// 安全除法，避免被除数为 0
pub fn safe_div(a: f64, b: f64) -> Option<f64> {
    if b.abs() < std::f64::EPSILON {
        None
    } else {
        Some(a / b)
    }
}
