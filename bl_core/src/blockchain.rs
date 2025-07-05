use url::Url;
use crate::errors::Result;

#[derive(Debug)]
pub struct Blockchain {
    name: String,
    docker_image: Url,
    http_api: Option<Url>,
    ws_api: Option<Url>,
}

pub trait BlockchainCluster {
    async fn create(&self) -> Result<()>;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    async fn delete(&self) -> Result<()>;
}

pub trait BlockchainNode {
    async fn create(&self) -> Result<()>;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    async fn delete(&self) -> Result<()>;
}

impl BlockchainNode for Blockchain {
    async fn create(&self) -> Result<()> {
        todo!()
    }

    async fn start(&self) -> Result<()> {
        todo!()
    }

    async fn stop(&self) -> Result<()> {
        todo!()
    }

    async fn delete(&self) -> Result<()> {
        todo!()
    }
}
