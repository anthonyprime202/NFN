use anyhow::Error as AnyhowError;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Internal(#[from] AnyhowError)
}


impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type AppResult<T> = std::result::Result<T, AppError>;
