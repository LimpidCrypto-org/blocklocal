use std::process::Command;

use clap::Parser;

use crate::{
    commands::errors::CommandError, core::bl_command::BLCommand, errors::Result,
    utils::processes::get_pid,
};

#[derive(Parser, Debug)]
pub struct StopArgs {}

impl StopArgs {
    pub fn stop_k3s() -> Result<()> {
        let pid = get_pid("k3s").unwrap();
        if let Err(error) = Command::new("sudo")
            .arg("kill")
            .arg(pid.to_string())
            .spawn()
        {
            return Err(CommandError::ClapError(error.into()).into());
        }

        Ok(())
    }
}

impl BLCommand for StopArgs {
    fn run(&self) -> Result<()> {
        StopArgs::stop_k3s()
    }
}
