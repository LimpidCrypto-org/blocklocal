pub mod orchestrator;

#[derive(Debug, thiserror::Error)]
pub enum StrategyError {
    #[error("Orchestrator error: {0}")]
    OrchestratorError(#[from] orchestrator::OrchestratorError),
}
