use std::{borrow::Cow, process::Command};

use crate::errors::Result;

use super::bl_output::BLOutput;

pub struct BLCommand {
    command: Command,
}

impl BLCommandFn for BLCommand {
    fn run(&mut self) -> Result<BLOutput> {
        let output = self.command.output()?;
        Ok(BLOutput::from(output))
    }
}

impl From<Command> for BLCommand {
    fn from(command: Command) -> Self {
        BLCommand { command }
    }
}

/// Defines the behavior of a command that can be executed by the BlockSim CLI
pub trait BLCommandFn {
    fn new(cmd: Cow<str>) -> BLCommand {
        let (cmd, args) = cmd.split_once(' ').unwrap_or((cmd.as_ref(), ""));
        let args = args.split(' ').collect::<Vec<&str>>();
        let mut command = Command::new(cmd);
        command.args(args);
        BLCommand { command }
    }
    /// Handles the execution of the command based on the passed arguments
    fn run(&mut self) -> Result<BLOutput>;
}
