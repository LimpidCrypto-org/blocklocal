use thiserror::Error;


#[derive(Debug, Error)]
pub enum BLOutputError {
    #[error("{0}")]
    StdErr(String)
}
