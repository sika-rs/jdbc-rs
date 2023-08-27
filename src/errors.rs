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

#[derive(Debug)]
pub enum InitError {
    JvmError(jni::JvmError),
    StartJvmError(jni::errors::StartJvmError),
    JniError(jni::errors::Error),
}

impl From<jni::JvmError> for InitError {
    fn from(err: jni::JvmError) -> Self {
        InitError::JvmError(err)
    }
}

impl From<jni::errors::StartJvmError> for InitError {
    fn from(err: jni::errors::StartJvmError) -> Self {
        InitError::StartJvmError(err)
    }
}

impl From<jni::errors::Error> for InitError {
    fn from(err: jni::errors::Error) -> Self {
        InitError::JniError(err)
    }
}

impl Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitError::JvmError(err) => err.fmt(f),
            InitError::StartJvmError(err) => err.fmt(f),
            InitError::JniError(err) => err.fmt(f),
        }
    }
}
