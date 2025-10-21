use crate::{
    command::{Args, Command},
    handler::handle_init,
};
use clap::Parser;

mod command;
mod handler;

fn main() -> Result<(), String> {
    let args = Args::parse();

    match args.command {
        Command::Init { name, description } => handle_init(name, description)?,
    }

    Ok(())
}
