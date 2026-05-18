
//! jrust-servo: jrust and Servo integration bridge

pub mod error;
pub mod event_channel;
pub mod dom_update;
pub mod servo_init;
pub mod snap_injector;

pub use error::{Error, Result};
pub use event_channel::{EventChannel, ServoEvent};
pub use dom_update::DomUpdate;
pub use servo_init::{ServoConfig, ServoInstance};
pub use snap_injector::SnapInjector;
