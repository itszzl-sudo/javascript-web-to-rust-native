//! Android View

use jrust_platform::ViewId;
use std::collections::HashMap;

/// Android view representation
pub struct AndroidView {
    pub id: ViewId,
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
    pub text_content: Option<String>,
    pub children: Vec<ViewId>,
    pub parent: Option<ViewId>,
}

impl AndroidView {
    pub fn new(id: ViewId, tag_name: &str) -> Self {
        Self {
            id,
            tag_name: tag_name.to_string(),
            attributes: HashMap::new(),
            text_content: None,
            children: Vec::new(),
            parent: None,
        }
    }
}
