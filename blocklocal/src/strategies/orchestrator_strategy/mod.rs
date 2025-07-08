use std::{fmt::Display, path::{Path, PathBuf}};

use crate::strategies::orchestrator_strategy::docker_compose_orchestrator_strategy::DockerComposeOrchestratorError;

pub mod docker_compose_orchestrator_strategy;

#[derive(Debug, thiserror::Error)]
pub enum OrchestratorError {
    #[error("Docker Compose error: {0}")]
    DockerComposeError(#[from] DockerComposeOrchestratorError),
}

pub struct Orchestrator<T: OrchestratorStrategy> {
    strategy: T,
    config: PathBuf,
}

impl<T: OrchestratorStrategy> Orchestrator<T> {
    /// Creates a new orchestrator with the given strategy.
    pub fn new(strategy: T, config: PathBuf) -> Self {
        Orchestrator { strategy, config }
    }

    /// Returns a reference to the strategy.
    pub fn get_strategy(&self) -> &T {
        &self.strategy
    }

    /// Returns a mutable reference to the strategy.
    pub fn get_strategy_mut(&mut self) -> &mut T {
        &mut self.strategy
    }

    /// Sets the strategy to a new one.
    pub fn set_strategy(&mut self, strategy: T) {
        self.strategy = strategy;
    }

    pub fn get_config(&self) -> &Path {
        &self.config
    }

    pub fn get_config_mut(&mut self) -> &mut PathBuf {
        &mut self.config
    }

    pub fn set_config(&mut self, config: PathBuf) {
        self.config = config;
    }
}

impl<T: OrchestratorStrategy> Orchestrator<T> {
    fn create(&self) -> Result<(), OrchestratorError> {
        self.strategy.create(&self.config)?;
        Ok(())
    }

    fn up(&self) -> Result<(), OrchestratorError> {
        self.strategy.up(&self.config)?;
        Ok(())
    }

    fn down(&self) -> Result<(), OrchestratorError> {
        self.strategy.down(&self.config)?;
        Ok(())
    }

    fn status(&self) -> Result<(), OrchestratorError> {
        self.strategy.status(&self.config)?;
        Ok(())
    }

    fn logs(&self) -> Result<(), OrchestratorError> {
        self.strategy.logs(&self.config)?;
        Ok(())
    }

    fn restart(&self) -> Result<(), OrchestratorError> {
        self.strategy.restart(&self.config)?;
        Ok(())
    }
}

pub trait OrchestratorStrategy {
    fn create(&self, config: &Path) -> Result<(), OrchestratorError>;
    fn up(&self, config: &Path) -> Result<(), OrchestratorError>;
    fn down(&self, config: &Path) -> Result<(), OrchestratorError>;
    fn status(&self, config: &Path) -> Result<(), OrchestratorError>;
    fn logs(&self, config: &Path) -> Result<(), OrchestratorError>;
    fn restart(&self, config: &Path) -> Result<(), OrchestratorError> {
        self.down(config)?;
        self.up(config)
    }
}

pub trait GetOrchestratorConfig {
    fn get_config(&self) -> &Path;
    fn get_config_mut(&mut self) -> &mut Path;
}

#[cfg(test)]
mod tests {
    use crate::strategies::orchestrator_strategy::docker_compose_orchestrator_strategy::DockerComposeOrchestratorStrategy;

    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_docker_compose_orchestrator() {
        // Example test for Docker Compose orchestrator
        let config_path = PathBuf::from("/workspaces/blocklocal/blocklocal/config/blockchains/xrpld-standalone-genesis/orchestrations/docker-compose/docker-compose.yaml");
        let orchestrator = Orchestrator::new(DockerComposeOrchestratorStrategy, config_path);

        orchestrator.up().unwrap();
    }
}
