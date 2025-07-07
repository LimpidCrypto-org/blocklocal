pub mod directory_service;
pub mod github_repo_service;
pub mod zip_service;

#[derive(Debug, thiserror::Error)]
/// Represents errors that can occur in the services.
pub enum ServiceError {
    #[error("GitHub repository service error: {0}")]
    GitHubRepoServiceError(#[from] github_repo_service::GitHubRepoServiceError),
    #[error("Zip service error: {0}")]
    ZipServiceError(#[from] zip_service::ZipServiceError),
}

impl From<github_repo_service::GitHubRepoServiceError> for crate::errors::Error {
    fn from(error: github_repo_service::GitHubRepoServiceError) -> Self {
        ServiceError::GitHubRepoServiceError(error).into()
    }
}

impl From<zip_service::ZipServiceError> for crate::errors::Error {
    fn from(error: zip_service::ZipServiceError) -> Self {
        ServiceError::ZipServiceError(error).into()
    }
}
