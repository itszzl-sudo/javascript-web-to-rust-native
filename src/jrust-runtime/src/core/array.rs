use std::fmt;
use crate::core::JsValue;

#[derive(Debug, Clone, PartialEq)]
pub struct JsArray {
    elements: Vec<JsValue>,
}

impl JsArray {
    pub fn new() -> Self {
        JsArray {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, value: JsValue) {
        self.elements.push(value);
    }

    pub fn pop(&mut self) -> Option<JsValue> {
        self.elements.pop()
    }

    pub fn shift(&mut self) -> Option<JsValue> {
        if !self.elements.is_empty() {
            Some(self.elements.remove(0))
        } else {
            None
        }
    }

    pub fn unshift(&mut self, value: JsValue) {
        self.elements.insert(0, value);
    }

    pub fn get(&self, index: usize) -> Option<JsValue> {
        self.elements.get(index).cloned()
    }

    pub fn set(&mut self, index: usize, value: JsValue) {
        if index < self.elements.len() {
            self.elements[index] = value;
        } else {
            self.elements.push(value);
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn slice(&self, start: usize, end: Option<usize>) -> JsArray {
        let len = self.elements.len();
        let start = if start > len { len } else { start };
        let end = end.unwrap_or(len);
        let end = if end > len { len } else if end < start { start } else { end };
        JsArray {
            elements: self.elements[start..end].to_vec(),
        }
    }

    pub fn to_vec(&self) -> Vec<JsValue> {
        self.elements.clone()
    }

    pub fn iter(&self) -> impl Iterator<Item = &JsValue> {
        self.elements.iter()
    }
}

impl IntoIterator for JsArray {
    type Item = JsValue;
    type IntoIter = std::vec::IntoIter<JsValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl From<Vec<JsValue>> for JsArray {
    fn from(elements: Vec<JsValue>) -> Self {
        JsArray { elements }
    }
}

impl Default for JsArray {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for JsArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[object Array]")
    }
}
