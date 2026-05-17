
use crate::director::jrust_tree::{JsRustId, JsRustInstance, JsRustTree};

/// Director - jrust 的指挥中心
pub struct Director {
    jrust_tree: JsRustTree,
}

impl Director {
    pub fn new() -> Self {
        Self {
            jrust_tree: JsRustTree::new(),
        }
    }
    
    pub fn add_jrust(&mut self, instance: Box<dyn JsRustInstance>) -> JsRustId {
        let id = self.jrust_tree.create_root(instance);
        if let Some(node) = self.jrust_tree.get_node_mut(id) {
            node.instance.init();
        }
        id
    }
    
    pub fn create_child_jrust(&mut self, parent_id: JsRustId, instance: Box<dyn JsRustInstance>) -> Option<JsRustId> {
        let id = self.jrust_tree.create_child(parent_id, instance)?;
        if let Some(node) = self.jrust_tree.get_node_mut(id) {
            node.instance.init();
        }
        Some(id)
    }
    
    pub fn dispatch_event(&mut self) {
        self.jrust_tree.dispatch_event();
    }
}

impl Default for Director {
    fn default() -> Self {
        Self::new()
    }
}

/// 简单的示例 jrust 实现
pub struct SimpleJsRust {
    name: String,
}

impl SimpleJsRust {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl JsRustInstance for SimpleJsRust {
    fn init(&mut self) {
        println!("{} 初始化完成", self.name);
    }
    
    fn handle_event(&mut self) -> bool {
        println!("{} 收到事件", self.name);
        false
    }
    
    fn deploy_javascript_task(&mut self, _js_code: &str) {}
    
    fn get_children(&self) -> Vec<JsRustId> {
        Vec::new()
    }
}

