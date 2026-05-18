//! # API Bindings
//! 
//! JavaScript → Rust API 绑定系统

pub mod svelte;
pub mod preact;
pub mod solid;
pub mod react;
pub mod angular;

use crate::core::{JsValue};
use std::collections::HashMap;

/// 绑定注册表
pub struct BindingRegistry {
    bindings: HashMap<String, Box<dyn Fn(&[JsValue]) -> Result<JsValue, String>>>,
}

impl BindingRegistry {
    pub fn new() -> Self {
        BindingRegistry {
            bindings: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, name: &str, f: F)
    where
        F: Fn(&[JsValue]) -> Result<JsValue, String> + 'static,
    {
        self.bindings.insert(name.to_string(), Box::new(f));
    }

    pub fn call(&self, name: &str, args: &[JsValue]) -> Result<JsValue, String> {
        if let Some(f) = self.bindings.get(name) {
            f(args)
        } else {
            Err(format!("Binding '{}' not found", name))
        }
    }
}

impl Default for BindingRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// 注册默认的 DOM 绑定
pub fn register_dom_bindings(registry: &mut BindingRegistry) {
    registry.register("document.createElement", |args| {
        if args.len() != 1 {
            return Err("createElement expects 1 argument".to_string());
        }
        let _tag_name = match &args[0] {
            JsValue::String(s) => s,
            _ => return Err("First argument must be a string".to_string()),
        };
        Ok(JsValue::new_object()) // TODO: 正确处理 Element
    });

    registry.register("document.getElementById", |args| {
        if args.len() != 1 {
            return Err("getElementById expects 1 argument".to_string());
        }
        let _id = match &args[0] {
            JsValue::String(s) => s,
            _ => return Err("First argument must be a string".to_string()),
        };
        Ok(JsValue::Undefined) // TODO: 实现
    });

    registry.register("element.appendChild", |args| {
        if args.len() != 1 {
            return Err("appendChild expects 1 argument".to_string());
        }
        Ok(JsValue::Undefined) // TODO: 实现
    });
}

/// 注册所有框架绑定
pub fn register_all_framework_bindings(registry: &mut BindingRegistry) {
    register_dom_bindings(registry);
    svelte::register_svelte_bindings(registry);
    preact::register_preact_bindings(registry);
    solid::register_solid_bindings(registry);
    react::register_react_bindings(registry);
    angular::register_angular_bindings(registry);
}

/// 转换 JsValue 为 &str
fn to_str<'a>(val: &'a JsValue) -> Result<&'a str, String> {
    match val {
        JsValue::String(s) => Ok(s),
        _ => Err("Expected string".to_string()),
    }
}

/// 转换 JsValue 为 f64
fn to_num(val: &JsValue) -> Result<f64, String> {
    match val {
        JsValue::Number(n) => Ok(*n),
        _ => Err("Expected number".to_string()),
    }
}

/// 转换 JsValue 为 bool
fn to_bool(val: &JsValue) -> Result<bool, String> {
    match val {
        JsValue::Boolean(b) => Ok(*b),
        _ => Err("Expected boolean".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binding_registry() {
        let mut registry = BindingRegistry::new();
        
        registry.register("test", |args| {
            if args.len() == 1 {
                Ok(args[0].clone())
            } else {
                Err("Wrong number of args".to_string())
            }
        });

        let result = registry.call("test", &[JsValue::new_number(42.0)]);
        assert_eq!(result, Ok(JsValue::new_number(42.0)));
    }
}
