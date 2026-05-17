
//! 事件通道: Servo 与 jruste 之间的双向通信

use crossbeam_channel::{unbounded, Sender, Receiver};
use crate::error::{Error, Result};
use crate::dom_update::DomUpdate;

/// Servo 事件（简化版，后续对接真实 Servo 事件）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServoEvent {
    /// 点击事件
    Click {
        target_id: Option<String>,
        x: i32,
        y: i32,
    },
    
    /// 输入事件
    Input {
        target_id: Option<String>,
        value: String,
    },
    
    /// 键盘事件
    KeyDown {
        key: String,
        key_code: u32,
    },
    
    /// 键盘事件
    KeyUp {
        key: String,
        key_code: u32,
    },
    
    /// 鼠标移动
    MouseMove {
        x: i32,
        y: i32,
    },
}

/// 事件通道
pub struct EventChannel {
    /// Servo → jruste
    pub servo_to_jruste: Sender<ServoEvent>,
    pub servo_to_jruste_rx: Receiver<ServoEvent>,
    
    /// jruste → Servo
    pub jruste_to_servo: Sender<DomUpdate>,
    pub jruste_to_servo_rx: Receiver<DomUpdate>,
}

impl EventChannel {
    /// 创建新的事件通道
    pub fn new() -> Self {
        let (servo_tx, servo_rx) = unbounded();
        let (jruste_tx, jruste_rx) = unbounded();
        
        Self {
            servo_to_jruste: servo_tx,
            servo_to_jruste_rx: servo_rx,
            jruste_to_servo: jruste_tx,
            jruste_to_servo_rx: jruste_rx,
        }
    }
    
    /// 从 Servo 发送事件到 jruste
    pub fn send_servo_event(&self, event: ServoEvent) -> Result<()> {
        self.servo_to_jruste.send(event)
            .map_err(|e| Error::EventSendError(format!("{}", e)))
    }
    
    /// 从 jruste 发送 DOM 更新到 Servo
    pub fn send_dom_update(&self, update: DomUpdate) -> Result<()> {
        self.jruste_to_servo.send(update)
            .map_err(|e| Error::EventSendError(format!("{}", e)))
    }
    
    /// 接收来自 Servo 的事件（jruste 端）
    pub fn recv_servo_event(&self) -> Result<ServoEvent> {
        self.servo_to_jruste_rx.recv()
            .map_err(|e| Error::EventSendError(format!("{}", e)))
    }
    
    /// 接收来自 jruste 的 DOM 更新（Servo 端）
    pub fn recv_dom_update(&self) -> Result<DomUpdate> {
        self.jruste_to_servo_rx.recv()
            .map_err(|e| Error::EventSendError(format!("{}", e)))
    }
}

impl Default for EventChannel {
    fn default() -> Self {
        Self::new()
    }
}
