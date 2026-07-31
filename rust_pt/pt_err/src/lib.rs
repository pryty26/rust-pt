use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ConfigError {
    #[error("Invalid {side} Config: {message}")]
    InvalidConfigErr { side: String, message: String },
}
