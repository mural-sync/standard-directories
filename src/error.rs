#[derive(Debug)]
pub enum Error {
    HomePath,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HomePath => write!(
                f,
                "failed to find the home directory. try setting the $HOME environment variable."
            ),
        }
    }
}

impl std::error::Error for Error {}
