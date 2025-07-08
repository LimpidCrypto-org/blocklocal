
use crate::{environment::Environment, managers::ManagerError};

pub struct TempDirManager;

impl TempDirManager {
    const BLOCKCHAIN_CONFIGS_DIR: &'static str = "blockchain-configs";

    const BLOCKCHAIN_CONFIG_HIERARCHY: &'static str = "<hash>/tmp";

    pub fn get_temp_dir() -> Result<String, ManagerError> {
        let temp_dir = Environment::new().get_app_tmp_dir();

        if !std::path::Path::new(&temp_dir).exists() {
            std::fs::create_dir_all(&temp_dir)?;
        }

        Ok(temp_dir)
    }

    pub fn get_blockchain_configs_dir() -> Result<String, ManagerError> {
        let temp_dir = Self::get_temp_dir()?;
        let blockchain_configs_dir = format!("{}/{}", temp_dir, Self::BLOCKCHAIN_CONFIGS_DIR);

        if !std::path::Path::new(&blockchain_configs_dir).exists() {
            std::fs::create_dir_all(&blockchain_configs_dir)?;
        }

        Ok(blockchain_configs_dir)
    }

    pub fn create_blockchain_config(hash: &str) -> Result<String, ManagerError> {
        let blockchain_configs_dir = Self::get_blockchain_configs_dir()?;
        let new_dir = format!("{}/{}/tmp", blockchain_configs_dir, hash);

        if !std::path::Path::new(&new_dir).exists() {
            std::fs::create_dir_all(&new_dir)?;
        }

        Ok(new_dir)
    }

    pub fn cleanup_blockchain_config(hash: &str) -> Result<(), ManagerError> {
        let blockchain_configs_dir = Self::get_blockchain_configs_dir()?;
        let temporary_directory_path = format!("{}/{}/tmp", blockchain_configs_dir, hash);

        if std::path::Path::new(&temporary_directory_path).exists() {
            std::fs::remove_dir_all(&temporary_directory_path)?;
        }

        Ok(())
    }
}
