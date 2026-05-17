use std::collections::HashMap;

pub struct RequestInit {
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl Default for RequestInit {
    fn default() -> Self {
        RequestInit {
            method: String::from("GET"),
            headers: HashMap::new(),
            body: None,
        }
    }
}

pub struct Response {
    pub ok: bool,
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
}

pub async fn fetch(url: &str, _init: Option<RequestInit>) -> Result<Response, String> {
    println!("Fetching URL: {}", url);
    Ok(Response {
        ok: true,
        status: 200,
        status_text: String::from("OK"),
        headers: HashMap::new(),
    })
}
