use thiserror::Error;
#[derive(Error, Debug)]
pub enum AppError {

    #[error("Network error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Unexpected error: {0}")]
    Other(String),
}

pub type AppResult<T> = Result<T, AppError>;