
//! DOM 更新指令

use serde::{Serialize, Deserialize};

/// DOM 更新指令
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DomUpdate {
    /// 创建元素
    CreateElement {
        parent_id: String,
        tag_name: String,
        new_id: Option<String>,
    },
    
    /// 删除元素
    RemoveElement {
        element_id: String,
    },
    
    /// 设置属性
    SetAttribute {
        element_id: String,
        name: String,
        value: String,
    },
    
    /// 移除属性
    RemoveAttribute {
        element_id: String,
        name: String,
    },
    
    /// 设置文本内容
    SetTextContent {
        element_id: String,
        text: String,
    },
    
    /// 设置 inner HTML
    SetInnerHtml {
        element_id: String,
        html: String,
    },
    
    /// 添加类名
    AddClass {
        element_id: String,
        class_name: String,
    },
    
    /// 移除类名
    RemoveClass {
        element_id: String,
        class_name: String,
    },
}

impl DomUpdate {
    /// 创建 CreateElement 指令
    pub fn create_element(
        parent_id: String,
        tag_name: String,
        new_id: Option<String>,
    ) -> Self {
        Self::CreateElement {
            parent_id,
            tag_name,
            new_id,
        }
    }
    
    /// 创建 RemoveElement 指令
    pub fn remove_element(element_id: String) -> Self {
        Self::RemoveElement {
            element_id,
        }
    }
    
    /// 创建 SetAttribute 指令
    pub fn set_attribute(
        element_id: String,
        name: String,
        value: String,
    ) -> Self {
        Self::SetAttribute {
            element_id,
            name,
            value,
        }
    }
    
    /// 创建 SetTextContent 指令
    pub fn set_text_content(
        element_id: String,
        text: String,
    ) -> Self {
        Self::SetTextContent {
            element_id,
            text,
        }
    }
}
