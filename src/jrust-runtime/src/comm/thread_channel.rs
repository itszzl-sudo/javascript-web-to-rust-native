
//! 跨线程通信通道
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};

/// 跨线程通信的消息类型
#[derive(Debug, Clone)]
pub enum ThreadMessage {
    /// 更新 DOM 文档
    UpdateDocument(String),

    /// 发送事件
    SendEvent(String),

    /// 调用 JS 函数
    CallFunction {
        name: String,
        args: Vec<String>,
    },

    /// 响应结果
    Response(Result<String, String>),

    /// 关闭通道
    Shutdown,
}

/// 跨线程通信通道
pub struct ThreadChannel {
    /// 发送端：jrust → runtime
    sender: Sender<ThreadMessage>,

    /// 接收端：runtime → jrust
    receiver: Arc<Mutex<Receiver<ThreadMessage>>>,
}

impl ThreadChannel {
    /// 创建新的跨线程通信通道
    pub fn new() -> (Self, Self) {
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        let channel1 = Self {
            sender: tx1,
            receiver: Arc::new(Mutex::new(rx2)),
        };

        let channel2 = Self {
            sender: tx2,
            receiver: Arc::new(Mutex::new(rx1)),
        };

        (channel1, channel2)
    }

    /// 发送消息
    pub fn send(&self, msg: ThreadMessage) -> Result<(), String> {
        self.sender.send(msg)
            .map_err(|e| format!("Failed to send message: {}", e))
    }

    /// 接收消息（阻塞）
    pub fn recv(&self) -> Result<ThreadMessage, String> {
        let rx = self.receiver.lock()
            .map_err(|e| format!("Failed to lock receiver: {}", e))?;
        rx.recv().map_err(|e| format!("Failed to receive message: {}", e))
    }

    /// 接收消息（非阻塞）
    pub fn try_recv(&self) -> Result<Option<ThreadMessage>, String> {
        let rx = self.receiver.lock()
            .map_err(|e| format!("Failed to lock receiver: {}", e))?;
        match rx.try_recv() {
            Ok(msg) => Ok(Some(msg)),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(e) => Err(format!("Failed to try receive: {}", e)),
        }
    }

    /// 更新文档
    pub fn update_document(&self, doc_json: String) -> Result<(), String> {
        self.send(ThreadMessage::UpdateDocument(doc_json))
    }

    /// 发送事件
    pub fn send_event(&self, event_json: String) -> Result<(), String> {
        self.send(ThreadMessage::SendEvent(event_json))
    }

    /// 调用函数
    pub fn call_function(&self, name: String, args: Vec<String>) -> Result<(), String> {
        self.send(ThreadMessage::CallFunction { name, args })
    }

    /// 发送响应
    pub fn send_response(&self, result: Result<String, String>) -> Result<(), String> {
        self.send(ThreadMessage::Response(result))
    }

    /// 关闭通道
    pub fn shutdown(&self) -> Result<(), String> {
        self.send(ThreadMessage::Shutdown)
    }
}

impl Default for ThreadChannel {
    fn default() -> Self {
        // 创建单向通道（用于默认初始化）
        let (tx, rx) = mpsc::channel();
        Self {
            sender: tx,
            receiver: Arc::new(Mutex::new(rx)),
        }
    }
}

