use reqwest::Client;
use url::Url;

use crate::{environment::Environment, strategies::blockchain_config_downloader_strategy::{BlockchainConfigDownloaderError, BlockchainConfigDownloaderStrategy}};

#[derive(Debug, thiserror::Error)]
/// Represents errors that can occur when downloading blockchain configurations from GitHub.
pub enum GithubBlockchainConfigDownloaderError {
    #[error("{0}")]
    Other(String),
    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),

}

pub struct GithubBlockchainConfigDownloaderStrategy;

impl GithubBlockchainConfigDownloaderStrategy {
    fn build_codeload_url(&self, url: &Url) -> Result<Url, BlockchainConfigDownloaderError> {
        let repo_owner = url
            .path_segments()
            .and_then(|mut segments| segments.nth(0))
            .ok_or_else(|| GithubBlockchainConfigDownloaderError::Other("Invalid URL".to_string()))?;
        let repo_name = url
            .path_segments()
            .and_then(|mut segments| segments.nth(1))
            .ok_or_else(|| GithubBlockchainConfigDownloaderError::Other("Invalid URL".to_string()))?;
        let branch = url
            .path_segments()
            .and_then(|mut segments| segments.nth(3))
            .ok_or_else(|| GithubBlockchainConfigDownloaderError::Other("Invalid URL".to_string()))?;
        let base_url = format!(
            "https://codeload.github.com/{}/{}/zip/refs/heads/{}",
            repo_owner, repo_name, branch
        );
        let url = Url::parse(&base_url).map_err(GithubBlockchainConfigDownloaderError::from)?;

        Ok(url)
    }
}

impl BlockchainConfigDownloaderStrategy for GithubBlockchainConfigDownloaderStrategy {
    fn _build_request(&self, url: &Url) -> Result<reqwest::Request, BlockchainConfigDownloaderError> {
        let client = Client::new();
        let codeload_url = self.build_codeload_url(url)?;
        let mut request_builder = client
            .get(codeload_url)
            .header("Accept", "application/vnd.github.v3+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("User-Agent", "blocklocal");
        let bearer = Environment::new().get_github_bearer();

        if bearer.is_some() {
            request_builder =
                request_builder.header("Authorization", format!("Bearer {}", bearer.unwrap()));
        }

        Ok(request_builder.build().map_err(|e| BlockchainConfigDownloaderError::ReqwestError(e))?)
    }
}