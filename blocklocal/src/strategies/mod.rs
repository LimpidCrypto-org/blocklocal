use crate::strategies::{blockchain_config_downloader_strategy::BlockchainConfigDownloaderError, orchestrator_strategy::OrchestratorError};

pub mod blockchain_config_downloader_strategy;
pub mod orchestrator_strategy;

#[derive(Debug, thiserror::Error)]
pub enum StrategyError {
    #[error("Blockchain config downloader error: {0}")]
    BlockchainConfigDownloaderError(#[from] BlockchainConfigDownloaderError),
    #[error("Orchestrator error: {0}")]
    OrchestratorError(#[from] OrchestratorError),
}
