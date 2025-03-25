#[cfg(not(target_os = "linux"))]
compile_error!("your operating system is not supported yet.");

mod error;
use std::path::PathBuf;

pub use error::Error;

#[cfg(target_os = "linux")]
mod linux;

/// Get the configuration directory.
pub fn config_path(
    #[cfg_attr(target_os = "linux", allow(unused))] organization_name: String,
    app_name: String,
) -> Result<PathBuf, Error> {
    #[cfg(target_os = "linux")]
    linux::config_path(app_name)
}

/// Get the data directory.
pub fn data_path(
    #[cfg_attr(target_os = "linux", allow(unused))] organization_name: String,
    app_name: String,
) -> Result<PathBuf, Error> {
    #[cfg(target_os = "linux")]
    linux::data_path(app_name)
}
