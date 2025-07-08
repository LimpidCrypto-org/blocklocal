use std::{
    env::set_var,
    path::{Path, PathBuf},
};

use crate::{
    strategies::orchestrator_strategy::{GetOrchestratorConfig, OrchestratorError, OrchestratorStrategy},
    utils::random::generate_random_string,
};

use compose_rs::{Compose, ComposeBuilder, ComposeBuilderError, ComposeCommand, ComposeError};

pub struct DockerComposeOrchestratorStrategy;

#[derive(Debug, thiserror::Error)]
pub enum DockerComposeOrchestratorError {
    #[error("Docker Compose command failed: {0}")]
    ComposeError(#[from] ComposeError),
    #[error("Docker Compose builder error: {0}")]
    ComposeBuilderError(#[from] ComposeBuilderError),
}

impl DockerComposeOrchestratorStrategy {
    fn build_compose_file(&self, config: &Path) -> Result<Compose, DockerComposeOrchestratorError> {
        ComposeBuilder::new()
            .path(config.to_str().ok_or_else(|| {
                DockerComposeOrchestratorError::ComposeBuilderError(
                    ComposeBuilderError::MissingField("config path".to_string()),
                )
            })?)
            .build()
            .map_err(DockerComposeOrchestratorError::from)
    }
}

impl OrchestratorStrategy for DockerComposeOrchestratorStrategy {
    fn create(&self, config: &Path) -> Result<(), OrchestratorError> {
        // Implementation for creating the Docker Compose services
        let compose = self.build_compose_file(config)?;
        compose.up().exec().map_err(DockerComposeOrchestratorError::from)?;

        todo!("Implement the create method for Docker Compose services");

        Ok(())
    }

    fn up(&self, config: &Path) -> Result<(), OrchestratorError> {
        // Implementation for bringing up the Docker Compose services
        let compose = self.build_compose_file(config)?;
        set_var("ID", generate_random_string(8)); // Set a random ID for the Docker Compose services
        compose.up().exec().map_err(DockerComposeOrchestratorError::from)?;
        set_var("ID", ""); // Clear the ID variable after use

        Ok(())
    }

    fn down(&self, config: &Path) -> Result<(), OrchestratorError> {
        // Implementation for bringing down the Docker Compose services
        Ok(())
    }

    fn status(&self, config: &Path) -> Result<(), OrchestratorError> {
        // Implementation for checking the status of the Docker Compose services
        Ok(())
    }

    fn logs(&self, config: &Path) -> Result<(), OrchestratorError> {
        // Implementation for fetching the logs of the Docker Compose services
        Ok(())
    }

    fn restart(&self, config: &Path) -> Result<(), OrchestratorError> {
        // Implementation for restarting the Docker Compose services
        Ok(())
    }
}
