
//! JavaScript 值类型系统

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use crate::core::{JsObject, JsArray, JsFunction};

/// JavaScript 值类型枚举，支持所有基本类型
#[derive(Debug, Clone, PartialEq)]
pub enum JsValue {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Object(Rc<RefCell<JsObject>>),
    Array(Rc<RefCell<JsArray>>),
    Function(Rc<RefCell<JsFunction>>),
}

impl fmt::Display for JsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsValue::Undefined => write!(f, "undefined"),
            JsValue::Null => write!(f, "null"),
            JsValue::Boolean(b) => write!(f, "{}", b),
            JsValue::Number(n) => write!(f, "{}", n),
            JsValue::String(s) => write!(f, "\"{}\"", s),
            JsValue::Object(_) => write!(f, "[object Object]"),
            JsValue::Array(_) => write!(f, "[object Array]"),
            JsValue::Function(_) => write!(f, "function"),
        }
    }
}

impl JsValue {
    pub fn new_undefined() -> Self {
        JsValue::Undefined
    }

    pub fn new_null() -> Self {
        JsValue::Null
    }

    pub fn new_boolean(b: bool) -> Self {
        JsValue::Boolean(b)
    }

    pub fn new_number(n: f64) -> Self {
        JsValue::Number(n)
    }

    pub fn new_string(s: impl Into<String>) -> Self {
        JsValue::String(s.into())
    }

    pub fn new_object() -> Self {
        JsValue::Object(Rc::new(RefCell::new(JsObject::new())))
    }

    pub fn new_array() -> Self {
        JsValue::Array(Rc::new(RefCell::new(JsArray::new())))
    }

    pub fn is_undefined(&self) -> bool {
        matches!(self, JsValue::Undefined)
    }

    pub fn is_null(&self) -> bool {
        matches!(self, JsValue::Null)
    }

    pub fn is_nullish(&self) -> bool {
        self.is_null() || self.is_undefined()
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, JsValue::Boolean(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, JsValue::Number(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, JsValue::String(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, JsValue::Object(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, JsValue::Array(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(self, JsValue::Function(_))
    }

    pub fn to_boolean(&self) -> bool {
        match self {
            JsValue::Undefined => false,
            JsValue::Null => false,
            JsValue::Boolean(b) => *b,
            JsValue::Number(n) => *n != 0.0 && !n.is_nan(),
            JsValue::String(s) => !s.is_empty(),
            JsValue::Object(_) => true,
            JsValue::Array(_) => true,
            JsValue::Function(_) => true,
        }
    }

    pub fn to_number(&self) -> f64 {
        match self {
            JsValue::Undefined => f64::NAN,
            JsValue::Null => 0.0,
            JsValue::Boolean(b) => if *b { 1.0 } else { 0.0 },
            JsValue::Number(n) => *n,
            JsValue::String(s) => s.parse().unwrap_or(f64::NAN),
            JsValue::Object(_) => f64::NAN,
            JsValue::Array(_) => f64::NAN,
            JsValue::Function(_) => f64::NAN,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            JsValue::Undefined => "undefined".to_string(),
            JsValue::Null => "null".to_string(),
            JsValue::Boolean(b) => b.to_string(),
            JsValue::Number(n) => n.to_string(),
            JsValue::String(s) => s.clone(),
            JsValue::Object(_) => "[object Object]".to_string(),
            JsValue::Array(_) => "[object Array]".to_string(),
            JsValue::Function(_) => "function".to_string(),
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            JsValue::Boolean(b) => Some(*b),
            _ => None
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            JsValue::Number(n) => Some(*n),
            _ => None
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            JsValue::String(s) => Some(s),
            _ => None
        }
    }

    pub fn as_object(&self) -> Option<Rc<RefCell<JsObject>>> {
        match self {
            JsValue::Object(o) => Some(Rc::clone(o)),
            _ => None
        }
    }

    pub fn as_array(&self) -> Option<Rc<RefCell<JsArray>>> {
        match self {
            JsValue::Array(a) => Some(Rc::clone(a)),
            _ => None
        }
    }
}

impl From<bool> for JsValue {
    fn from(b: bool) -> Self {
        JsValue::new_boolean(b)
    }
}

impl From<f64> for JsValue {
    fn from(n: f64) -> Self {
        JsValue::new_number(n)
    }
}

impl From<i32> for JsValue {
    fn from(n: i32) -> Self {
        JsValue::new_number(n as f64)
    }
}

impl From<&str> for JsValue {
    fn from(s: &str) -> Self {
        JsValue::new_string(s)
    }
}

impl From<String> for JsValue {
    fn from(s: String) -> Self {
        JsValue::new_string(s)
    }
}
