pub mod errors;
// mod get;
// mod create;
mod start;
// mod stop;

// use get::args::GetArgs;
// use start::args::StartArgs;
// use stop::args::StopArgs;

use clap::Subcommand;
use start::args::StartArgs;

use crate::{
    core::{bl_command::BLCommandFn, bl_output::BLOutput},
    errors::Result,
};

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Start the blocksim server
    Start(StartArgs),
    // /// Stop the blocksim server
    // Stop(StopArgs),
    // /// Get information about the blocksim server
    // Get(GetArgs),
}

impl BLCommandFn for Commands {
    fn run(&mut self) -> Result<BLOutput> {
        match self {
            Commands::Start(start_args) => start_args.run(), // Commands::Stop(stop_args) => {
                                                             //     stop_args.run()?;
                                                             // }
                                                             // Commands::Get(get_args) => {
                                                             //     get_args.run()?;
                                                             // }
        }
    }
}
