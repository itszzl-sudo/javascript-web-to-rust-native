pub mod detector;
pub mod converter;
pub mod integration;

pub use detector::{CompilePath, PathDetector};
pub use converter::TypeInferencer;
pub use integration::TsNativeCompiler;

pub type Result<T> = anyhow::Result<T>;
