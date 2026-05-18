
//! 通信模块：提供跨线程和跨进程通信能力
pub mod thread_channel;
pub mod process_channel;

pub use thread_channel::{ThreadChannel, ThreadMessage};
pub use process_channel::{ProcessChannel, ProcessMessage, ProcessChannelServer, ProcessChannelStream};

/// 通信方式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommMode {
    /// 直接调用（默认）
    Direct,

    /// 跨线程通信
    Thread,

    /// 跨进程通信
    Process,
}

impl CommMode {
    /// 从字符串解析
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "direct" => Some(Self::Direct),
            "thread" => Some(Self::Thread),
            "process" => Some(Self::Process),
            _ => None,
        }
    }

    /// 转为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Thread => "thread",
            Self::Process => "process",
        }
    }
}
