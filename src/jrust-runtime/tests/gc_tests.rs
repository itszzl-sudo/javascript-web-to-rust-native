
use jrust_runtime::core::*;

#[test]
fn test_gc_allocation() {
    let mut gc = GarbageCollector::new();
    
    let id = gc.allocate(JsValue::new_number(42.0));
    
    assert_eq!(gc.current_objects_count(), 1);
    assert!(gc.get(id).is_some());
    
    let value = gc.get(id).unwrap();
    assert_eq!(value.to_number(), 42.0);
}

#[test]
fn test_gc_get_mut() {
    let mut gc = GarbageCollector::new();
    
    let id = gc.allocate(JsValue::new_number(10.0));
    
    if let Some(value) = gc.get_mut(id) {
        *value = JsValue::new_number(20.0);
    }
    
    let updated = gc.get(id).unwrap();
    assert_eq!(updated.to_number(), 20.0);
}

#[test]
fn test_gc_basic_collection() {
    let mut gc = GarbageCollector::new();
    
    let id1 = gc.allocate(JsValue::new_string("object1"));
    let id2 = gc.allocate(JsValue::new_string("object2"));
    
    assert_eq!(gc.current_objects_count(), 2);
    
    gc.remove_root(id1);
    let freed = gc.collect();
    
    assert_eq!(freed, 1);
    assert_eq!(gc.current_objects_count(), 1);
    assert!(gc.get(id1).is_none());
    assert!(gc.get(id2).is_some());
}

#[test]
fn test_gc_multiple_collections() {
    let mut gc = GarbageCollector::new();
    
    for i in 0..10 {
        gc.allocate(JsValue::new_number(i as f64));
    }
    
    assert_eq!(gc.current_objects_count(), 10);
    
    // 收集所有对象（因为没有根）
    for i in 0..10 {
        gc.remove_root(GcId::new(i + 1));
    }
    
    let freed = gc.collect();
    assert_eq!(freed, 10);
    assert_eq!(gc.current_objects_count(), 0);
}

#[test]
fn test_gc_add_root() {
    let mut gc = GarbageCollector::new();
    
    let id = gc.allocate(JsValue::new_undefined());
    gc.remove_root(id);
    
    // 重新添加根
    gc.add_root(id);
    
    let freed = gc.collect();
    assert_eq!(freed, 0);
    assert_eq!(gc.current_objects_count(), 1);
}

#[test]
fn test_gc_stats() {
    let mut gc = GarbageCollector::new();
    
    for _ in 0..5 {
        gc.allocate(JsValue::new_null());
    }
    
    assert_eq!(gc.stats().total_allocated, 5);
    assert_eq!(gc.stats().total_freed, 0);
    assert_eq!(gc.stats().collections_count, 0);
    
    for i in 0..5 {
        gc.remove_root(GcId::new(i + 1));
    }
    
    gc.collect();
    
    assert_eq!(gc.stats().total_freed, 5);
    assert_eq!(gc.stats().collections_count, 1);
}

#[test]
fn test_gc_should_collect() {
    let mut gc = GarbageCollector::new();
    
    for i in 0..8 {
        gc.allocate(JsValue::new_number(i as f64));
    }
    
    assert!(!gc.should_collect(10));
    
    gc.allocate(JsValue::new_number(8.0));
    gc.allocate(JsValue::new_number(9.0));
    
    assert!(gc.should_collect(10));
}

#[test]
fn test_gc_id_display() {
    let id = GcId::new(42);
    assert_eq!(id.to_string(), "GC-ID:42");
}
