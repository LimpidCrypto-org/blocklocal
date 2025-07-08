pub mod directory_manager;
pub mod temp_dir_manager;

#[derive(Debug, thiserror::Error)]
/// Represents errors that can occur in the managers.
pub enum ManagerError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
