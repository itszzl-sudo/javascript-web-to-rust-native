use std::fmt;
use indexmap::IndexMap;
use crate::core::JsValue;

#[derive(Debug, Clone, PartialEq)]
pub struct JsObject {
    properties: IndexMap<String, JsValue>,
}

impl JsObject {
    pub fn new() -> Self {
        JsObject {
            properties: IndexMap::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: JsValue) {
        self.properties.insert(key.into(), value);
    }

    pub fn get(&self, key: &str) -> Option<JsValue> {
        self.properties.get(key).cloned()
    }

    pub fn has(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.properties.swap_remove(key).is_some()
    }

    pub fn keys(&self) -> Vec<String> {
        self.properties.keys().cloned().collect()
    }

    pub fn values(&self) -> Vec<JsValue> {
        self.properties.values().cloned().collect()
    }

    pub fn entries(&self) -> Vec<(String, JsValue)> {
        self.properties.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    pub fn len(&self) -> usize {
        self.properties.len()
    }

    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }
}

impl Default for JsObject {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for JsObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[object Object]")
    }
}
