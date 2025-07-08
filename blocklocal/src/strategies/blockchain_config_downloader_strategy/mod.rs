use std::path::Path;

#[derive(Debug, thiserror::Error)]
/// Represents errors that can occur when downloading blockchain configurations.
pub enum BlockchainConfigDownloaderError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct BlockchainConfigDownloader<T: BlockchainConfigDownloaderStrategy> {
    strategy: T,
}

pub trait BlockchainConfigDownloaderStrategy {
    fn _build_request(&self, url: &str) -> Result<reqwest::Request, BlockchainConfigDownloaderError>;
}

impl<T: BlockchainConfigDownloaderStrategy> BlockchainConfigDownloader<T> {
    pub fn new(strategy: T) -> Self {
        Self { strategy }
    }

    pub fn set_strategy(&mut self, strategy: T) {
        self.strategy = strategy;
    }

    pub fn get_strategy(&self) -> &T {
        &self.strategy
    }

    pub fn get_strategy_mut(&mut self) -> &mut T {
        &mut self.strategy
    }
}

impl<T: BlockchainConfigDownloaderStrategy> BlockchainConfigDownloader<T> {
    /// Downloads the blockchain configuration from the given URL.
    pub async fn download_config(&self, url: &str) -> Result<reqwest::Response, BlockchainConfigDownloaderError> {
        let request = self.strategy._build_request(url)?;
        let client = reqwest::Client::new();
        let response = client.execute(request).await?;

        Ok(response)
    }
}