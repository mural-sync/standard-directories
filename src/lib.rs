mod error;
pub use error::Error;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{config_path, data_path};
