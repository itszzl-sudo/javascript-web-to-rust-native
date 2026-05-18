use std::fmt;
use std::rc::Rc;
use crate::core::JsValue;

pub type NativeFunction = fn(&[JsValue]) -> JsValue;
pub type ClosureFunction = Rc<dyn Fn(&[JsValue]) -> Result<JsValue, String>>;

#[derive(Clone)]
pub enum JsFunction {
    Native(NativeFunction),
    Closure(ClosureFunction),
}

impl PartialEq for JsFunction {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl JsFunction {
    pub fn new_native(func: NativeFunction) -> Self {
        JsFunction::Native(func)
    }

    pub fn new_closure<F>(f: F) -> Self
    where
        F: Fn(&[JsValue]) -> Result<JsValue, String> + 'static,
    {
        JsFunction::Closure(Rc::new(f))
    }

    pub fn call(&self, args: &[JsValue]) -> Result<JsValue, String> {
        match self {
            JsFunction::Native(func) => Ok(func(args)),
            JsFunction::Closure(func) => func(args),
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
