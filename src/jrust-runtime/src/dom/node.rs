
pub enum NodeType {
    Element,
    Text,
    Comment,
}

pub struct Node {
    node_type: NodeType,
    name: String,
}

impl Node {
    pub fn new_element(name: &str) -> Self {
        Node {
            node_type: NodeType::Element,
            name: name.to_string(),
        }
    }

    pub fn new_text(text: &str) -> Self {
        Node {
            node_type: NodeType::Text,
            name: text.to_string(),
        }
    }

    pub fn node_type(&self) -> &NodeType {
        &self.node_type
    }

    pub fn node_name(&self) -> &str {
        &self.name
    }
}
