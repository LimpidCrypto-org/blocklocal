use crate::managers::ManagerError;

pub struct DirectoryManager;

impl DirectoryManager {
    pub fn copy_directory(source: &str, destination: &str) -> Result<(), ManagerError> {
        if !std::path::Path::new(source).exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Source directory '{}' does not exist", source),
            )
            .into());
        }

        if !std::path::Path::new(destination).exists() {
            std::fs::create_dir_all(destination)?;
        }

        for entry in std::fs::read_dir(source)? {
            let entry = entry?;
            let path = entry.path();
            let dest_path = std::path::PathBuf::from(destination).join(entry.file_name());

            if path.is_dir() {
                DirectoryManager::copy_directory(
                    path.to_str().unwrap(),
                    dest_path.to_str().unwrap(),
                )?;
            } else {
                std::fs::copy(&path, &dest_path)?;
            }
        }
        Ok(())
    }

    pub fn delete_directory(path: &str) -> Result<(), ManagerError> {
        if std::path::Path::new(path).exists() {
            std::fs::remove_dir_all(path)?;
        }
        Ok(())
    }
}
