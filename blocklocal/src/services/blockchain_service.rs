use std::path::Path;

use crate::{managers::ManagerError, services::ServiceError};

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
    const BLOCKCHAIN_CONFIG_HIERARCHY: &'static [&str] = &[
        "<hash>/tmp", // Temporary directory for downloading, extracting, and processing blockchain configurations
        "<hash>/config", // Directory containing the blockchain configuration files
        "<hash>/orchestrations",
        "<hash>/orchestrations/docker-compose"
    ];

    /// Validates the blockchain configuration hierarchy for a specific config hash.
    pub fn validate_blockchain_config_hierarchy(
        hash: &str,
        config: &Path,
    ) -> Result<(), ServiceError> {
        Self::BLOCKCHAIN_CONFIG_HIERARCHY
            .iter()
            .try_for_each(|&dir| {
                let path = config.join(dir.replace("<hash>", hash));
                if !path.exists() {
                    Err(BlockchainServiceError::InvalidConfig(format!(
                        "Missing directory: {}",
                        path.display()
                    )))
                } else {
                    Ok(())
                }
            })
            .map_err(ServiceError::from)?;

        Ok(())
    }
}