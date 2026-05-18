
//! 跨进程通信通道
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

/// 跨进程通信的消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessMessage {
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

/// 跨进程通信通道 - 服务器端（Runtime）
pub struct ProcessChannelServer {
    listener: TcpListener,
    addr: SocketAddr,
    clients: Arc<Mutex<Vec<TcpStream>>>,
}

impl ProcessChannelServer {
    /// 创建新的服务器并开始监听
    pub fn bind(addr: &str) -> Result<Self, String> {
        let listener = TcpListener::bind(addr)
            .map_err(|e| format!("Failed to bind: {}", e))?;
        let addr = listener.local_addr()
            .map_err(|e| format!("Failed to get local addr: {}", e))?;
        
        println!("✅ Process channel server listening on: {}", addr);

        Ok(Self {
            listener,
            addr,
            clients: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// 获取绑定的地址
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// 等待并接受一个客户端连接
    pub fn accept(&self) -> Result<ProcessChannelStream, String> {
        let (stream, addr) = self.listener.accept()
            .map_err(|e| format!("Failed to accept: {}", e))?;
        println!("✅ New client connected: {}", addr);
        
        let stream_clone = stream.try_clone()
            .map_err(|e| format!("Failed to clone stream: {}", e))?;
        self.clients.lock()
            .map_err(|e| format!("Failed to lock clients: {}", e))?
            .push(stream_clone);

        Ok(ProcessChannelStream::from_stream(stream))
    }

    /// 广播消息给所有连接的客户端
    pub fn broadcast(&self, msg: ProcessMessage) -> Result<(), String> {
        let data = serde_json::to_vec(&msg)
            .map_err(|e| format!("Failed to serialize: {}", e))?;
        
        let mut clients = self.clients.lock()
            .map_err(|e| format!("Failed to lock clients: {}", e))?;
        
        let mut disconnected = Vec::new();
        for (i, client) in clients.iter_mut().enumerate() {
            if let Err(e) = client.write_all(&data) {
                eprintln!("❌ Failed to send to client {}: {}", i, e);
                disconnected.push(i);
            }
            // 发送分隔符
            let _ = client.write_all(b"\nEND\n");
        }

        // 移除断开的连接
        for &i in disconnected.iter().rev() {
            clients.remove(i);
        }

        Ok(())
    }
}

/// 跨进程通信通道 - 客户端流
pub struct ProcessChannelStream {
    stream: Arc<Mutex<TcpStream>>,
}

impl ProcessChannelStream {
    /// 从 TcpStream 创建
    pub fn from_stream(stream: TcpStream) -> Self {
        Self {
            stream: Arc::new(Mutex::new(stream)),
        }
    }

    /// 连接到服务器
    pub fn connect(addr: &str) -> Result<Self, String> {
        let stream = TcpStream::connect(addr)
            .map_err(|e| format!("Failed to connect: {}", e))?;
        println!("✅ Connected to process channel server: {}", addr);
        Ok(Self::from_stream(stream))
    }

    /// 发送消息
    pub fn send(&self, msg: ProcessMessage) -> Result<(), String> {
        let data = serde_json::to_vec(&msg)
            .map_err(|e| format!("Failed to serialize: {}", e))?;
        
        let mut stream = self.stream.lock()
            .map_err(|e| format!("Failed to lock stream: {}", e))?;
        
        stream.write_all(&data)
            .map_err(|e| format!("Failed to write: {}", e))?;
        stream.write_all(b"\nEND\n")
            .map_err(|e| format!("Failed to write delimiter: {}", e))?;
        stream.flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;
        
        Ok(())
    }

    /// 接收消息
    pub fn recv(&self) -> Result<ProcessMessage, String> {
        let mut stream = self.stream.lock()
            .map_err(|e| format!("Failed to lock stream: {}", e))?;
        
        let mut buffer = Vec::new();
        let mut temp_buf = [0; 4096];
        
        loop {
            let n = stream.read(&mut temp_buf)
                .map_err(|e| format!("Failed to read: {}", e))?;
            
            if n == 0 {
                return Err("Connection closed".to_string());
            }
            
            buffer.extend_from_slice(&temp_buf[..n]);
            
            // 检查是否有分隔符
            if let Some(pos) = buffer.windows(5).position(|w| w == b"\nEND\n") {
                // 先克隆数据，然后再修改 buffer
                let data = buffer[..pos].to_vec();
                buffer.drain(..pos + 5);
                
                let msg = serde_json::from_slice(&data)
                    .map_err(|e| format!("Failed to deserialize: {}", e))?;
                
                return Ok(msg);
            }
        }
    }

    /// 更新文档
    pub fn update_document(&self, doc_json: String) -> Result<(), String> {
        self.send(ProcessMessage::UpdateDocument(doc_json))
    }

    /// 发送事件
    pub fn send_event(&self, event_json: String) -> Result<(), String> {
        self.send(ProcessMessage::SendEvent(event_json))
    }

    /// 调用函数
    pub fn call_function(&self, name: String, args: Vec<String>) -> Result<(), String> {
        self.send(ProcessMessage::CallFunction { name, args })
    }

    /// 发送响应
    pub fn send_response(&self, result: Result<String, String>) -> Result<(), String> {
        self.send(ProcessMessage::Response(result))
    }

    /// 关闭通道
    pub fn shutdown(&self) -> Result<(), String> {
        self.send(ProcessMessage::Shutdown)
    }
}

/// 简单的跨进程通信包装器
pub struct ProcessChannel {
    stream: ProcessChannelStream,
}

impl ProcessChannel {
    /// 作为客户端连接
    pub fn connect(addr: &str) -> Result<Self, String> {
        Ok(Self {
            stream: ProcessChannelStream::connect(addr)?,
        })
    }

    /// 发送消息
    pub fn send(&self, msg: ProcessMessage) -> Result<(), String> {
        self.stream.send(msg)
    }

    /// 接收消息
    pub fn recv(&self) -> Result<ProcessMessage, String> {
        self.stream.recv()
    }
}

