use std::{fmt::Display, path::Path};

pub mod docker_compose_strategy;

pub struct Orchestrator<T: OrchestratorStrategy> {
    strategy: T,
}

impl<T: OrchestratorStrategy> Orchestrator<T> {
    /// Creates a new orchestrator with the given strategy.
    pub fn new(strategy: T) -> Self {
        Orchestrator { strategy }
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
}

impl<T: OrchestratorStrategy> OrchestratorStrategy for Orchestrator<T> {
    type Error = T::Error;

    fn create(&self) -> Result<(), Self::Error> {
        self.strategy.create()
    }

    fn up(&self) -> Result<(), Self::Error> {
        self.strategy.up()
    }

    fn down(&self) -> Result<(), Self::Error> {
        self.strategy.down()
    }

    fn status(&self) -> Result<(), Self::Error> {
        self.strategy.status()
    }

    fn logs(&self) -> Result<(), Self::Error> {
        self.strategy.logs()
    }

    fn restart(&self) -> Result<(), Self::Error> {
        self.strategy.restart()
    }
}

pub trait OrchestratorStrategy {
    type Error: Display;

    fn create(&self) -> Result<(), Self::Error>;
    fn up(&self) -> Result<(), Self::Error>;
    fn down(&self) -> Result<(), Self::Error>;
    fn status(&self) -> Result<(), Self::Error>;
    fn logs(&self) -> Result<(), Self::Error>;
    fn restart(&self) -> Result<(), Self::Error> {
        self.down()?;
        self.up()
    }
}

pub trait GetOrchestratorConfig {
    fn get_config(&self) -> &Path;
    fn get_config_mut(&mut self) -> &mut Path;
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_docker_compose_orchestrator() {
        // Example test for Docker Compose orchestrator
        let config_path = PathBuf::from("/workspaces/blocklocal/blocklocal/config/nodes/xrpld-standalone-genesis/orchestrations/docker-compose/docker-compose.yaml");
        let strategy = docker_compose_strategy::DockerComposeOrchestratorStrategy::new(config_path);
        let orchestrator = Orchestrator::new(strategy);

        orchestrator.up().unwrap();
    }
}
