
//! 错误类型定义

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Snap 加载失败: {0}")]
    SnapLoadError(String),

    #[error("Snap 转换失败: {0}")]
    SnapConversionError(String),

    #[error("Servo 初始化失败: {0}")]
    ServoInitError(String),

    #[error("事件发送失败: {0}")]
    EventSendError(String),

    #[error("DOM 更新失败: {0}")]
    DomUpdateError(String),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON 错误: {0}")]
    JsonError(#[from] serde_json::Error),
}
