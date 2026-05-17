
pub struct Navigator {
    pub user_agent: String,
    pub language: String,
}

impl Navigator {
    pub fn new() -> Self {
        Navigator {
            user_agent: String::from("javascript-web-runtime/0.2.0"),
            language: String::from("en-US"),
        }
    }
}

impl Default for Navigator {
    fn default() -> Self {
        Self::new()
    }
}
