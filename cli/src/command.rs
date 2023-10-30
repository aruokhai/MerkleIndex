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
    #[arg(short, long)]
    pub url: String,

    #[arg(short, long)]
    pub user: String,
    /// Subcommands
    #[clap(subcommand)]
    pub command: Commands,
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


