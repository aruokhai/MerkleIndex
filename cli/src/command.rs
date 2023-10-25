use std::{path::PathBuf, str::FromStr, io::Error};
use clap::{Parser, Subcommand};

use crate::connector::register_provider;

#[derive(Parser, Debug)]
#[clap(
    name = "merkleIndex cli",
    author,
    about,
    long_about = "Merkle Index CLI",
    version
)]

pub struct Cli {
    /// Subcommands
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(
        name = "register_provider",
        about = "Register A Provider",
        long_about = None, 
    )]
    RegisterProvider,
}



pub fn cli_match() -> Result<(), Error> {
    // Parse the command line arguments
    let cli = Cli::parse();

    // Merge clap config file if the value is set
    
    // Execute the subcommand
    match &cli.command {
        Commands::RegisterProvider => register_provider()?,
    }

    Ok(())
}