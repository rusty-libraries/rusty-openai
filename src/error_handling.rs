use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpenAIError {
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] ReqwestError),

    #[error("Serde JSON Error: {0}")]
    SerdeJsonError(#[from] SerdeJsonError),

    #[error("IO Error: {0}")]
    IoError(#[from] IoError),
}

pub type OpenAIResult<T> = std::result::Result<T, OpenAIError>;
