pub mod github_blockchain_config_downloader_strategy;

use std::path::Path;

use url::Url;

use crate::strategies::blockchain_config_downloader_strategy::github_blockchain_config_downloader_strategy::GithubBlockchainConfigDownloaderError;

#[derive(Debug, thiserror::Error)]
/// Represents errors that can occur when downloading blockchain configurations.
pub enum BlockchainConfigDownloaderError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("GitHub blockchain config downloader error: {0}")]
    GithubBlockchainConfigDownloaderError(#[from] GithubBlockchainConfigDownloaderError),
}

pub struct BlockchainConfigDownloader<T: BlockchainConfigDownloaderStrategy> {
    strategy: T,
}

pub trait BlockchainConfigDownloaderStrategy {
    fn _build_request(&self, url: &Url) -> Result<reqwest::Request, BlockchainConfigDownloaderError>;
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
    pub async fn download_config(&self, url: &Url) -> Result<reqwest::Response, BlockchainConfigDownloaderError> {
        let request = self.strategy._build_request(url)?;
        let client = reqwest::Client::new();
        let response = client.execute(request).await?;

        Ok(response)
    }
}