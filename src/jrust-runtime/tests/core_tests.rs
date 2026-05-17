
use jrust_runtime::core::*;

#[test]
fn test_js_value_creation() {
    // 测试基本类型创建
    let undef = JsValue::new_undefined();
    let null = JsValue::new_null();
    let boolean = JsValue::new_boolean(true);
    let number = JsValue::new_number(42.0);
    let string = JsValue::new_string("hello");
    let object = JsValue::new_object();
    let array = JsValue::new_array();

    // 验证类型检查
    assert!(undef.is_undefined());
    assert!(null.is_null());
    assert!(boolean.is_boolean());
    assert!(number.is_number());
    assert!(string.is_string());
    assert!(object.is_object());
    assert!(array.is_array());
}

#[test]
fn test_js_value_conversions() {
    // 测试 to_boolean
    assert_eq!(JsValue::new_undefined().to_boolean(), false);
    assert_eq!(JsValue::new_null().to_boolean(), false);
    assert_eq!(JsValue::new_boolean(false).to_boolean(), false);
    assert_eq!(JsValue::new_boolean(true).to_boolean(), true);
    assert_eq!(JsValue::new_number(0.0).to_boolean(), false);
    assert_eq!(JsValue::new_number(42.0).to_boolean(), true);
    assert_eq!(JsValue::new_string("").to_boolean(), false);
    assert_eq!(JsValue::new_string("hello").to_boolean(), true);

    // 测试 to_number
    assert!(JsValue::new_undefined().to_number().is_nan());
    assert_eq!(JsValue::new_null().to_number(), 0.0);
    assert_eq!(JsValue::new_boolean(true).to_number(), 1.0);
    assert_eq!(JsValue::new_number(42.0).to_number(), 42.0);

    // 测试 to_string
    assert_eq!(JsValue::new_undefined().to_string(), "undefined");
    assert_eq!(JsValue::new_null().to_string(), "null");
    assert_eq!(JsValue::new_boolean(true).to_string(), "true");
    assert_eq!(JsValue::new_number(42.0).to_string(), "42");
    assert_eq!(JsValue::new_string("hello").to_string(), "hello");
}

#[test]
fn test_js_object_basics() {
    let mut obj = JsObject::new();
    assert!(obj.is_empty());
    assert_eq!(obj.len(), 0);

    // 测试 set 和 get
    obj.set("name", JsValue::new_string("John"));
    assert_eq!(obj.len(), 1);
    assert!(obj.has("name"));
    
    let name = obj.get("name");
    assert!(name.is_some());
    assert_eq!(name.unwrap().to_string(), "John");

    // 测试 keys, values, entries
    obj.set("age", JsValue::new_number(30.0));
    assert_eq!(obj.keys().len(), 2);
    assert_eq!(obj.values().len(), 2);
    assert_eq!(obj.entries().len(), 2);

    // 测试 delete
    assert!(obj.delete("name"));
    assert_eq!(obj.len(), 1);
    assert!(!obj.has("name"));
}

#[test]
fn test_js_array_basics() {
    let mut arr = JsArray::new();
    assert!(arr.is_empty());
    assert_eq!(arr.len(), 0);

    // 测试 push
    arr.push(JsValue::new_number(1.0));
    arr.push(JsValue::new_number(2.0));
    assert_eq!(arr.len(), 2);

    // 测试 get 和 set
    let val = arr.get(0);
    assert!(val.is_some());
    assert_eq!(val.unwrap().to_number(), 1.0);

    arr.set(0, JsValue::new_number(10.0));
    let new_val = arr.get(0);
    assert_eq!(new_val.unwrap().to_number(), 10.0);

    // 测试 pop
    let popped = arr.pop();
    assert!(popped.is_some());
    assert_eq!(popped.unwrap().to_number(), 2.0);
    assert_eq!(arr.len(), 1);

    // 测试 shift 和 unshift
    arr.unshift(JsValue::new_number(5.0));
    assert_eq!(arr.len(), 2);
    
    let shifted = arr.shift();
    assert!(shifted.is_some());
    assert_eq!(shifted.unwrap().to_number(), 5.0);
    assert_eq!(arr.len(), 1);
}

#[test]
fn test_js_array_slice() {
    let mut arr = JsArray::new();
    for i in 0..5 {
        arr.push(JsValue::new_number(i as f64));
    }

    // 测试正常情况
    let slice1 = arr.slice(1, Some(4));
    assert_eq!(slice1.len(), 3);

    // 测试超出边界的安全处理
    let slice2 = arr.slice(10, None);
    assert_eq!(slice2.len(), 0);

    // 测试 end < start 的安全处理
    let slice3 = arr.slice(3, Some(1));
    assert_eq!(slice3.len(), 0);
}
