use std::{
    io::Write, os::unix::process::CommandExt, process::{Command, Output}
};

use clap::Parser;

use crate::{
    commands::errors::CommandError, core::bl_command::BLCommand, errors::Result,
};

use super::subcommands::GetCommands;

#[derive(Debug, Parser)]
pub struct GetArgs {
    #[command(subcommand)]
    pub command: GetCommands,
}

impl GetArgs {
    pub fn get_k3s_nodes() -> Result<()> {
        match Command::new("sudo")
            .arg("k3d")
            .arg("")
            .arg("get")
            .arg("nod")
            .output()
        {
            Ok(output) => {
                // write the output to stdout
                std::io::stdout().write_all(&output.stdout).unwrap();
                Ok(())
            }
            Err(error) => return Err(CommandError::ClapError(error.into()).into()),
        }
    }
}

impl BLCommand for GetArgs {
    fn run(&self) -> Result<()> {
        match &self.command {
            GetCommands::Nodes => GetArgs::get_k3s_nodes()?,
            GetCommands::Clusters => println!("Getting clusters"),
        }

        Ok(())
    }
}
