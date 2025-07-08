use crate::services::{blockchain_service::BlockchainServiceError, github_repo_service::GitHubRepoServiceError, zip_service::ZipServiceError};

pub mod blockchain_service;
pub mod github_repo_service;
pub mod zip_service;

#[derive(Debug, thiserror::Error)]
/// Represents errors that can occur in the services.
pub enum ServiceError {
    #[error("Blockchain service error: {0}")]
    BlockchainServiceError(#[from] BlockchainServiceError),
    #[error("GitHub repository service error: {0}")]
    GitHubRepoServiceError(#[from] GitHubRepoServiceError),
    #[error("Zip service error: {0}")]
    ZipServiceError(#[from] ZipServiceError),
}