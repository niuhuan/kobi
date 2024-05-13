use std::backtrace::Backtrace;
use std::fmt::{Display, Formatter};

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub struct Error {
    pub backtrace: Backtrace,
    pub info: ErrorInfo,
}

#[derive(Debug)]
pub enum ErrorInfo {
    Network(reqwest::Error),
    Message(String),
    Convert(serde_json::Error),
    Other(Box<dyn std::error::Error + Sync + Send>),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_struct("copy_client::Error");
        match &self.info {
            ErrorInfo::Convert(err) => {
                builder.field("kind", &"Convert");
                builder.field("source", err);
            }
            ErrorInfo::Network(err) => {
                builder.field("kind", &"Network");
                builder.field("source", err);
            }
            ErrorInfo::Message(err) => {
                builder.field("kind", &"Message");
                builder.field("source", err);
            }
            ErrorInfo::Other(err) => {
                builder.field("kind", &"Other");
                builder.field("source", err);
            }
        }
        builder.finish()
    }
}

impl Error {
    pub(crate) fn message(content: impl Into<String>) -> Self {
        Self {
            backtrace: Backtrace::capture(),
            info: ErrorInfo::Message(content.into()),
        }
    }
}

macro_rules! from_error {
    ($error_type:ty, $info_type:path) => {
        impl From<$error_type> for Error {
            fn from(value: $error_type) -> Self {
                Self {
                    backtrace: Backtrace::capture(),
                    info: $info_type(value),
                }
            }
        }
    };
}

from_error!(::reqwest::Error, ErrorInfo::Network);
from_error!(::serde_json::Error, ErrorInfo::Convert);
