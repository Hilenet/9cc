#[derive(Debug, Clone)]
pub enum Error {
    TokenNonExists(String),
    UnexpectedToken(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::Error::*;
        match self {
            TokenNonExists(s) => write!(f, "expected token but not exists: {}", s),
            UnexpectedToken(s) => write!(f, "found unexpected token: {}", s),
        }
    }
}

impl std::error::Error for Error {}
