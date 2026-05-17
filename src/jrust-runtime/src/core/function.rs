use std::fmt;
use crate::core::JsValue;

pub type NativeFunction = fn(&[JsValue]) -> JsValue;

#[derive(Clone)]
pub enum JsFunction {
    Native(NativeFunction),
}

impl PartialEq for JsFunction {
    fn eq(&self, _other: &Self) -> bool {
        // Function pointers can't be compared in a meaningful way for our use case
        true
    }
}

impl JsFunction {
    pub fn new_native(func: NativeFunction) -> Self {
        JsFunction::Native(func)
    }

    pub fn call(&self, args: &[JsValue]) -> JsValue {
        match self {
            JsFunction::Native(func) => func(args),
        }
    }
}

impl fmt::Debug for JsFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "function")
    }
}

impl fmt::Display for JsFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "function")
    }
}
