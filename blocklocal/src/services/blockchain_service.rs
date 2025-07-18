use std::path::Path;

use crate::{managers::ManagerError, services::ServiceError, strategies::blockchain_config_downloader_strategy::BlockchainConfigDownloaderStrategy, utils::random::generate_random_string};

#[derive(Debug, thiserror::Error)]
/// Represents errors that can occur in the blockchain service.
pub enum BlockchainServiceError {
    #[error("The configuration hierarchy is invalid: {0}")]
    InvalidConfig(String),
    #[error("Manager error: {0}")]
    ManagerError(#[from] ManagerError),
}

pub struct BlockchainService;

impl BlockchainService {
    const BLOCKCHAIN_UNQ_HASH_LENGTH: usize = 16;

    /// Downloads the blockchain configuration from the specified URL.
    pub fn download_blockchain_config<T: BlockchainConfigDownloaderStrategy>(
        url: &str,
        strategy: T,
    ) -> Result<(), ServiceError> {
        let unq_hash = generate_random_string(16);
        Ok(())
    }
}