use std::{env::set_var, path::{Path, PathBuf}};

use crate::{strategies::orchestrator::{GetOrchestratorConfig, OrchestratorStrategy}, utils::random::generate_random_string};

use compose_rs::{Compose, ComposeBuilder, ComposeBuilderError, ComposeCommand, ComposeError};

pub struct DockerComposeOrchestratorStrategy {
    config: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum DockerComposeOrchestratorError {
    #[error("Docker Compose command failed: {0}")]
    ComposeError(#[from] ComposeError),
    #[error("Docker Compose builder error: {0}")]
    ComposeBuilderError(#[from] ComposeBuilderError),
}

impl DockerComposeOrchestratorStrategy {
    pub fn new(config: PathBuf) -> Self {
        DockerComposeOrchestratorStrategy { config }
    }

    fn build_compose_file(&self) -> Result<Compose, DockerComposeOrchestratorError> {
        ComposeBuilder::new()
            .path(self.config.to_str().ok_or_else(|| {
                DockerComposeOrchestratorError::ComposeBuilderError(
                    ComposeBuilderError::MissingField("config path".to_string()),
                )
            })?)
            .build()
            .map_err(DockerComposeOrchestratorError::from)
    }
}

impl GetOrchestratorConfig for DockerComposeOrchestratorStrategy {
    fn get_config(&self) -> &Path {
        &self.config
    }

    fn get_config_mut(&mut self) -> &mut Path {
        &mut self.config
    }
}

impl OrchestratorStrategy for DockerComposeOrchestratorStrategy {
    type Error = DockerComposeOrchestratorError;

    fn create(&self) -> Result<(), Self::Error> {
        // Implementation for creating the Docker Compose services
        let compose = self.build_compose_file()?;

        todo!("Implement the create method for Docker Compose services");

        Ok(())
    }

    fn up(&self) -> Result<(), Self::Error> {
        // Implementation for bringing up the Docker Compose services
        let compose = self.build_compose_file()?;
        set_var("ID", generate_random_string(8)); // Set a random ID for the Docker Compose services
        compose.up().exec()?;
        set_var("ID", ""); // Clear the ID variable after use

        Ok(())
    }

    fn down(&self) -> Result<(), Self::Error> {
        // Implementation for bringing down the Docker Compose services
        Ok(())
    }

    fn status(&self) -> Result<(), Self::Error> {
        // Implementation for checking the status of the Docker Compose services
        Ok(())
    }

    fn logs(&self) -> Result<(), Self::Error> {
        // Implementation for fetching the logs of the Docker Compose services
        Ok(())
    }

    fn restart(&self) -> Result<(), Self::Error> {
        // Implementation for restarting the Docker Compose services
        Ok(())
    }
}


