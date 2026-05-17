
use std::collections::{HashMap, HashSet};
use std::fmt;
use crate::core::JsValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GcId(u64);

impl GcId {
    pub fn new(id: u64) -> Self {
        GcId(id)
    }
}

impl fmt::Display for GcId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GC-ID:{}", self.0)
    }
}

pub struct GcObject {
    value: JsValue,
    marked: bool,
}

impl GcObject {
    fn new(_id: GcId, value: JsValue) -> Self {
        GcObject {
            value,
            marked: false,
        }
    }
}

pub struct GarbageCollector {
    objects: HashMap<GcId, GcObject>,
    roots: HashSet<GcId>,
    next_id: u64,
    stats: GcStats,
}

#[derive(Debug, Default)]
pub struct GcStats {
    pub total_allocated: usize,
    pub total_freed: usize,
    pub collections_count: usize,
}

impl GarbageCollector {
    pub fn new() -> Self {
        GarbageCollector {
            objects: HashMap::new(),
            roots: HashSet::new(),
            next_id: 1,
            stats: GcStats::default(),
        }
    }

    pub fn allocate(&mut self, value: JsValue) -> GcId {
        let id = GcId(self.next_id);
        self.next_id += 1;
        
        let obj = GcObject::new(id, value);
        self.objects.insert(id, obj);
        
        self.roots.insert(id);
        
        self.stats.total_allocated += 1;
        
        id
    }

    pub fn add_root(&mut self, id: GcId) {
        if self.objects.contains_key(&id) {
            self.roots.insert(id);
        }
    }

    pub fn remove_root(&mut self, id: GcId) {
        self.roots.remove(&id);
    }

    pub fn get(&self, id: GcId) -> Option<&JsValue> {
        self.objects.get(&id).map(|obj| &obj.value)
    }

    pub fn get_mut(&mut self, id: GcId) -> Option<&mut JsValue> {
        self.objects.get_mut(&id).map(|obj| &mut obj.value)
    }

    pub fn collect(&mut self) -> usize {
        self.stats.collections_count += 1;
        
        for obj in self.objects.values_mut() {
            obj.marked = false;
        }
        
        let mut stack: Vec<GcId> = self.roots.iter().copied().collect();
        
        while let Some(id) = stack.pop() {
            if let Some(obj) = self.objects.get_mut(&id) {
                if !obj.marked {
                    obj.marked = true;
                    stack.extend(Self::trace_references_static(&obj.value));
                }
            }
        }
        
        let mut freed = 0;
        let mut to_remove = Vec::new();
        
        for (id, obj) in &self.objects {
            if !obj.marked {
                to_remove.push(*id);
                freed += 1;
            }
        }
        
        for id in &to_remove {
            self.objects.remove(id);
            self.roots.remove(id);
        }
        
        self.stats.total_freed += freed;
        
        freed
    }

    fn trace_references_static(_value: &JsValue) -> Vec<GcId> {
        Vec::new()
    }

    pub fn stats(&self) -> &GcStats {
        &self.stats
    }

    pub fn current_objects_count(&self) -> usize {
        self.objects.len()
    }

    pub fn should_collect(&self, threshold: usize) -> bool {
        self.objects.len() >= threshold
    }
}

impl Default for GarbageCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gc_basic() {
        let mut gc = GarbageCollector::new();
        
        let id1 = gc.allocate(JsValue::new_number(42.0));
        let id2 = gc.allocate(JsValue::new_string("hello"));
        
        assert_eq!(gc.current_objects_count(), 2);
        
        gc.remove_root(id1);
        gc.collect();
        
        assert_eq!(gc.current_objects_count(), 1);
        assert!(gc.get(id1).is_none());
        assert!(gc.get(id2).is_some());
    }

    #[test]
    fn test_gc_stats() {
        let mut gc = GarbageCollector::new();
        
        let id1 = gc.allocate(JsValue::new_undefined());
        let id2 = gc.allocate(JsValue::new_null());
        
        gc.remove_root(id1);
        gc.remove_root(id2);
        
        let freed = gc.collect();
        
        assert_eq!(freed, 2);
        assert_eq!(gc.stats().total_freed, 2);
        assert_eq!(gc.current_objects_count(), 0);
    }
}
