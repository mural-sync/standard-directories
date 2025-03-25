use std::path::{Path, PathBuf};

use crate::Error;

pub fn config_path(app_name: String) -> Result<PathBuf, Error> {
    Ok(std::env::var("XDG_CONFIG_HOME")
        .map(|raw_config_home_path| Path::new(&raw_config_home_path).to_path_buf())
        .unwrap_or(home_path()?.join(".config"))
        .join(convert_app_name(app_name)))
}

pub fn data_path(app_name: String) -> Result<PathBuf, Error> {
    Ok(std::env::var("XDG_DATA_HOME")
        .map(|raw_data_home_path| Path::new(&raw_data_home_path).to_path_buf())
        .unwrap_or(home_path()?.join(".local/share"))
        .join(convert_app_name(app_name)))
}

fn home_path() -> Result<PathBuf, Error> {
    std::env::var("HOME")
        .map(|raw_home_path| Path::new(&raw_home_path).to_path_buf())
        .map_err(|_| Error::HomePath)
}

fn convert_app_name(app_name: String) -> String {
    app_name.to_lowercase().replace(" ", "-")
}
