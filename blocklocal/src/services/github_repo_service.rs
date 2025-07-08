use reqwest::{Client, StatusCode};
use url::Url;

use crate::{environment::Environment, services::ServiceError};

#[derive(Debug, thiserror::Error)]
/// Represents errors that can occur when interacting with the GitHub repository service.
pub enum GitHubRepoServiceError {
    #[error("Unauthorized access: {0}")]
    Forbidden(String),
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),
}

pub struct GitHubRepoService;

impl GitHubRepoService {
    /// Fetches repository data from GitHub.
    pub async fn fetch_repo_data(&self, url: Url) -> Result<String, ServiceError> {
        // let client = Client::new();
        // let codeload_url = self.build_codeload_url(url.clone())?;
        // let mut request_builder = client
        //     .get(codeload_url)
        //     .header("Accept", "application/vnd.github.v3+json")
        //     .header("X-GitHub-Api-Version", "2022-11-28")
        //     .header("User-Agent", "blocklocal");
        // let bearer = Environment::new().get_github_bearer();

        // if bearer.is_some() {
        //     request_builder =
        //         request_builder.header("Authorization", format!("Bearer {}", bearer.unwrap()));
        // }
        // let response = request_builder.send().await?;

        // if response.status().is_success() {
        //     let content = response.text().await?;

        //     Ok(content)
        // } else if response.status() == StatusCode::FORBIDDEN {
        //     Err(GitHubRepoServiceError::Forbidden(response.text().await?).into())
        // } else if response.status() == StatusCode::NOT_FOUND {
        //     Err(GitHubRepoServiceError::NotFound(response.text().await?).into())
        // } else {
        //     Err(GitHubRepoServiceError::Unexpected(response.text().await?).into())
        // }
        todo!("move to GithubBlockchainConfigDownloaderStrategy");
    }

    /// Builds a GitHub codeload URL for a HTML URL.
    fn build_codeload_url(&self, url: Url) -> Result<Url, ServiceError> {
        let repo_owner = url
            .path_segments()
            .and_then(|mut segments| segments.nth(0))
            .ok_or_else(|| GitHubRepoServiceError::Unexpected("Invalid URL".to_string()))?;
        let repo_name = url
            .path_segments()
            .and_then(|mut segments| segments.nth(1))
            .ok_or_else(|| GitHubRepoServiceError::Unexpected("Invalid URL".to_string()))?;
        let branch = url
            .path_segments()
            .and_then(|mut segments| segments.nth(3))
            .ok_or_else(|| GitHubRepoServiceError::Unexpected("Invalid URL".to_string()))?;
        let base_url = format!(
            "https://codeload.github.com/{}/{}/zip/refs/heads/{}",
            repo_owner, repo_name, branch
        );
        let url = Url::parse(&base_url).map_err(GitHubRepoServiceError::from)?;
        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[tokio::test]
    async fn test_fetch_repo_data() {
        let service = GitHubRepoService;
        let url = Url::parse("https://codeload.github.com/LimpidCrypto-org/blocklocal-blockchain-configs/zip/refs/heads/main").unwrap();
        let result = service.fetch_repo_data(url).await;
        assert!(
            result.is_ok(),
            "Expected successful fetch, got error: {:?}",
            result
        );
    }
}
