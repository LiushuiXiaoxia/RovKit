use serde_json::{Error, Value};

use serde::de::DeserializeOwned;
use serde::Serialize;

/// 将结构体序列化为 JSON 字符串
pub fn to_json<T: Serialize>(value: &T) -> Result<String, Error> {
    serde_json::to_string(value)
}
/// 将结构体序列化为 JSON 字符串（带缩进）
pub fn to_pretty_json<T: Serialize>(value: &T) -> Result<String, Error> {
    serde_json::to_string_pretty(value)
}

/// 从 JSON 字符串反序列化为结构体
pub fn from_json<T: DeserializeOwned>(s: &str) -> Result<T, Error> {
    serde_json::from_str(s)
}

/// 将 JSON 字符串解析为 serde_json::Value
pub fn parse(json_str: &str) -> Result<Value, Error> {
    serde_json::from_str(json_str)
}

/// 将 serde_json::Value 转为 JSON 字符串（压缩）
pub fn to_string(value: &Value) -> Result<String, Error> {
    serde_json::to_string(value)
}

/// 将 serde_json::Value 转为 JSON 字符串（带缩进）
pub fn to_pretty_string(value: &Value) -> Result<String, Error> {
    serde_json::to_string_pretty(value)
}

/// 获取对象中某个键的值
pub fn get<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    value.get(key)
}

/// 设置对象中的某个键值（返回新的 Value）
pub fn set(mut value: Value, key: &str, val: Value) -> Value {
    if let Value::Object(ref mut map) = value {
        map.insert(key.to_string(), val);
    }
    value
}

/// 判断是否是数组类型
pub fn is_array(value: &Value) -> bool {
    value.is_array()
}

/// 判断是否是对象类型
pub fn is_object(value: &Value) -> bool {
    value.is_object()
}

/// 判断是否是字符串类型
pub fn is_string(value: &Value) -> bool {
    value.is_string()
}
