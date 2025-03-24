use std::path::{Path, PathBuf};

use crate::Error;

pub fn config_path(
    #[allow(unused)] organization_name: String,
    app_name: String,
) -> Result<PathBuf, Error> {
    home_path().map(|home_path| home_path.join(".config").join(convert_app_name(app_name)))
}

pub fn data_path(
    #[allow(unused)] organization_name: String,
    app_name: String,
) -> Result<PathBuf, Error> {
    home_path().map(|home_path| {
        home_path
            .join(".local")
            .join("share")
            .join(convert_app_name(app_name))
    })
}

fn home_path() -> Result<PathBuf, Error> {
    std::env::var("HOME")
        .map(|raw_home_path| Path::new(&raw_home_path).to_path_buf())
        .map_err(|_| Error::HomePath)
}

fn convert_app_name(app_name: String) -> String {
    app_name.to_lowercase().replace(" ", "-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_path_test() {
        unsafe { std::env::set_var("HOME", "/home/john") };
        let expected = Path::new("/home/john/.config/standard-directories").to_path_buf();
        let actual = config_path("Foo".to_string(), "Standard Directories".to_string()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn data_path_test() {
        unsafe { std::env::set_var("HOME", "/home/john") };
        let expected = Path::new("/home/john/.local/share/standard-directories").to_path_buf();
        let actual = data_path("Foo".to_string(), "Standard Directories".to_string()).unwrap();
        assert_eq!(expected, actual);
    }
}
