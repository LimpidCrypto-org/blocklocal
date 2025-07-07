use crate::errors::Result;

#[derive(Debug, thiserror::Error)]
/// Represents errors that can occur when interacting with the Zip service.
pub enum ZipServiceError {
    #[error("Failed to open zip file: {0}")]
    ZipError(#[from] zip::result::ZipError),
}

pub struct ZipService;

impl ZipService {
    pub fn extract_zip_into(zip_path: &str, target_dir: &str) -> Result<()> {
        let zip_file = std::fs::File::open(zip_path)?;
        let mut archive = zip::ZipArchive::new(zip_file).map_err(ZipServiceError::ZipError)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(ZipServiceError::ZipError)?;
            let outpath = std::path::PathBuf::from(target_dir).join(file.mangled_name());

            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    std::fs::create_dir_all(p)?;
                }
                let mut outfile = std::fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }
        Ok(())
    }
}
