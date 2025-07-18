use url::Url;

use crate::{errors::Error, strategies::blockchain_config_downloader_strategy::BlockchainConfigDownloaderStrategy};

pub struct BlockchainFacade;

impl BlockchainFacade {
    pub fn download_config<T: BlockchainConfigDownloaderStrategy>(
        url: &Url,
        strategy: T,
    ) -> Result<(), Error> {
        Ok(())
    }
}