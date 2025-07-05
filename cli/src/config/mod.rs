use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BLServer {
    pub port: u16,
}
