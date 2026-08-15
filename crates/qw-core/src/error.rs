use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuietWatchError {
    UnsupportedPlatform(&'static str),
    DeviceNotFound(String),
    Backend(String),
    InvalidConfig(String),
}

impl fmt::Display for QuietWatchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPlatform(name) => {
                write!(formatter, "audio backend `{name}` is not available on this platform")
            }
            Self::DeviceNotFound(id) => write!(formatter, "audio device not found: {id}"),
            Self::Backend(message) => write!(formatter, "audio backend error: {message}"),
            Self::InvalidConfig(message) => write!(formatter, "invalid config: {message}"),
        }
    }
}

impl std::error::Error for QuietWatchError {}
