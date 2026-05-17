
pub struct Location {
    pub href: String,
    pub protocol: String,
    pub host: String,
    pub hostname: String,
    pub port: String,
    pub pathname: String,
    pub search: String,
    pub hash: String,
}

impl Location {
    pub fn new() -> Self {
        Location {
            href: String::from("http://localhost/"),
            protocol: String::from("http:"),
            host: String::from("localhost"),
            hostname: String::from("localhost"),
            port: String::new(),
            pathname: String::from("/"),
            search: String::new(),
            hash: String::new(),
        }
    }

    pub fn reload(&self) {
        println!("Location.reload() called");
    }

    pub fn replace(&self, url: &str) {
        println!("Location.replace({}) called", url);
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}
