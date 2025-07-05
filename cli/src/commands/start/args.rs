
use clap::Parser;

use crate::{
    core::{
        bl_command::BLCommandFn,
        bl_output::BLOutput,
    },
    errors::Result,
};

#[derive(Parser, Debug)]
pub struct StartArgs {}

impl StartArgs {
    pub fn start_blocklocal_server() -> Result<BLOutput> {
        todo!()
    }

    pub fn add_blocklocal_redis_node() -> Result<BLOutput> {
        todo!()
    }
}

impl BLCommandFn for StartArgs {
    fn run(&mut self) -> Result<BLOutput> {
        StartArgs::start_blocklocal_server()
    }
}
