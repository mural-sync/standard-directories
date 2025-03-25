#[cfg(not(target_os = "linux"))]
compile_error!("your operating system is not supported yet.");

mod error;
pub use error::Error;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{config_path, data_path};
