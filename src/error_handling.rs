use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use std::fmt;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum OpenAIError {
    ReqwestError(ReqwestError),
    SerdeJsonError(SerdeJsonError),
    IoError(IoError),
}

impl fmt::Display for OpenAIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenAIError::ReqwestError(e) => write!(f, "Reqwest Error: {}", e),
            OpenAIError::SerdeJsonError(e) => write!(f, "Serde JSON Error: {}", e),
            OpenAIError::IoError(e) => write!(f, "IO Error: {}", e),
        }
    }
}

impl From<ReqwestError> for OpenAIError {
    fn from(error: ReqwestError) -> Self {
        OpenAIError::ReqwestError(error)
    }
}

impl From<SerdeJsonError> for OpenAIError {
    fn from(error: SerdeJsonError) -> Self {
        OpenAIError::SerdeJsonError(error)
    }
}

impl From<IoError> for OpenAIError {
    fn from(error: IoError) -> Self {
        OpenAIError::IoError(error)
    }
}