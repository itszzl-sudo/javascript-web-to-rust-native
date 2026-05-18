use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JsRustId(pub u64);

impl fmt::Display for JsRustId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "jrust-{}", self.0)
    }
}

pub trait JsRustInstance: Send + 'static {
    fn init(&mut self);
    fn handle_event(&mut self) -> bool;
    fn deploy_javascript_task(&mut self, js_code: &str);
    fn get_children(&self) -> Vec<JsRustId>;
}

pub struct JsRustNode {
    pub id: JsRustId,
    pub instance: Box<dyn JsRustInstance>,
    pub children: Vec<JsRustId>,
}

pub struct JsRustTree {
    nodes: HashMap<JsRustId, JsRustNode>,
    root: Option<JsRustId>,
    next_id: u64,
}

impl JsRustTree {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root: None,
            next_id: 1,
        }
    }
    
    pub fn create_root(&mut self, instance: Box<dyn JsRustInstance>) -> JsRustId {
        let id = JsRustId(self.next_id);
        self.next_id += 1;
        
        let node = JsRustNode {
            id,
            instance,
            children: Vec::new(),
        };
        
        self.nodes.insert(id, node);
        self.root = Some(id);
        id
    }
    
    pub fn create_child(&mut self, parent_id: JsRustId, instance: Box<dyn JsRustInstance>) -> Option<JsRustId> {
        if !self.nodes.contains_key(&parent_id) {
            return None;
        }
        
        let id = JsRustId(self.next_id);
        self.next_id += 1;
        
        let node = JsRustNode {
            id,
            instance,
            children: Vec::new(),
        };
        
        self.nodes.insert(id, node);
        
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(id);
        }
        
        Some(id)
    }
    
    pub fn get_node(&self, id: JsRustId) -> Option<&JsRustNode> {
        self.nodes.get(&id)
    }
    
    pub fn get_node_mut(&mut self, id: JsRustId) -> Option<&mut JsRustNode> {
        self.nodes.get_mut(&id)
    }
    
    pub fn get_root(&self) -> Option<JsRustId> {
        self.root
    }
    
    pub fn list_all_ids(&self) -> Vec<JsRustId> {
        self.nodes.keys().cloned().collect()
    }
    
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    pub fn dispatch_event(&mut self) -> bool {
        if let Some(root_id) = self.root {
            self.dispatch_event_recursive(root_id)
        } else {
            false
        }
    }
    
    fn dispatch_event_recursive(&mut self, current_id: JsRustId) -> bool {
        let mut stop_propagation = false;
        
        if let Some(node) = self.get_node_mut(current_id) {
            stop_propagation = node.instance.handle_event();
        }
        
        if !stop_propagation {
            let children: Vec<JsRustId> = {
                if let Some(node) = self.get_node(current_id) {
                    node.children.clone()
                } else {
                    Vec::new()
                }
            };
            
            for &child_id in &children {
                if let Some(child_node) = self.get_node_mut(child_id) {
                    child_node.instance.handle_event();
                }
            }
            
            for &child_id in &children {
                if !stop_propagation {
                    stop_propagation = self.dispatch_event_recursive(child_id);
                }
            }
        }
        
        stop_propagation
    }
    
    pub fn remove_node(&mut self, id: JsRustId) -> bool {
        if !self.nodes.contains_key(&id) {
            return false;
        }
        
        let children: Vec<JsRustId> = {
            if let Some(node) = self.nodes.get(&id) {
                node.children.clone()
            } else {
                Vec::new()
            }
        };
        
        for child_id in children {
            self.remove_node(child_id);
        }
        
        if self.root == Some(id) {
            self.root = None;
        }
        
        self.nodes.remove(&id);
        true
    }
    
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.root = None;
        self.next_id = 1;
    }
}

impl Default for JsRustTree {
    fn default() -> Self {
        Self::new()
    }
}
