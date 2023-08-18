use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    JniError(jni::errors::Error),
    ImpossibleError,
}

impl From<jni::errors::Error> for Error {
    fn from(err: jni::errors::Error) -> Self {
        Error::JniError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::JniError(err) => err.fmt(f),
            Error::ImpossibleError => f.write_str("Impossible Error."),
        }
    }
}
