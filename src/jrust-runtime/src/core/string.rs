
use crate::core::JsValue;

pub struct JsString;

impl JsString {
    pub fn from(s: &str) -> JsValue {
        JsValue::new_string(s)
    }

    pub fn concat(a: &str, b: &str) -> String {
        format!("{}{}", a, b)
    }

    pub fn length(s: &str) -> usize {
        s.len()
    }

    pub fn slice(s: &str, start: usize, end: Option<usize>) -> String {
        let end = end.unwrap_or(s.len());
        s[start..end].to_string()
    }

    pub fn to_upper_case(s: &str) -> String {
        s.to_uppercase()
    }

    pub fn to_lower_case(s: &str) -> String {
        s.to_lowercase()
    }

    pub fn trim(s: &str) -> String {
        s.trim().to_string()
    }
}
