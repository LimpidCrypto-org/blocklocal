
use thiserror::Error;

// use super::{get::errors::CmdGetError, start::errors::CmdStartError, stop::errors::CmdStopError};

#[derive(Debug, Error)]
#[error(transparent)]
pub enum CommandError {
    // CmdStartError(#[from] CmdStartError),
    // CmdStopError(#[from] CmdStopError),
    // CmdGetError(#[from] CmdGetError),
    ClapError(#[from] clap::Error),
}

