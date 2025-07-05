use thiserror::Error;

use crate::{commands::errors::CommandError, core::errors::BLOutputError};

pub type Result<T> = std::result::Result<T, BLError>;

#[derive(Error, Debug)]
#[error(transparent)]
pub enum BLError {
    CommandError(#[from] CommandError),
    OutputError(#[from] BLOutputError),
    IOError(#[from] std::io::Error),
    SerdeJsonError(#[from] serde_json::Error),
}
