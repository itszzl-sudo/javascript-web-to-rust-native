
//! jrust-browser: JRust + rust-browser integration bridge
//! 
//! This module provides integration between JRust and rust-browser

pub mod error;
pub mod bridge;

pub use error::{Error, Result};
pub use bridge::{BrowserConfig, BrowserInstance, JrustBrowserEvent, HttpResponse};
