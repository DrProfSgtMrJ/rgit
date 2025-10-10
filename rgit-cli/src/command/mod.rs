use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    // Init an empty rgit repository
    Init {
        #[arg(short, long)]
        // name of the rgit repository
        name: PathBuf,
        #[arg(short, long)]
        // description of the repository
        description: Option<String>,
    },
}
