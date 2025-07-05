mod commands;
mod config;
mod core;
mod errors;
mod utils;

use std::io::Write;

use clap::{error::ErrorKind, Error, Parser};
use commands::Commands;
use core::{bl_command::BLCommandFn, bl_output::BLOutputFn};
use errors::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut cli = BLCli::parse();

    match cli.command.run() {
        Ok(output) => {
            if cli.json {
                std::io::stdout().write_all(&serde_json::to_vec(&output.json()?)?)?;
            } else {
                std::io::stdout().write_all(output.to_string().as_bytes())?;
            }
        }
        Err(error) => Error::raw(ErrorKind::Io, error).exit(),
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "bl")]
#[command(version, about, long_about = None)]
pub struct BLCli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long)]
    pub json: bool,
}
