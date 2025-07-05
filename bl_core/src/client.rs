use crate::errors::Result;

pub trait ClientIO {
    async fn write(&mut self, request: &[u8]) -> Result<usize>;

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}
