use clap::Parser;

use crate::{
    command::{Args, Command},
    handler::handle_init,
};

mod command;
mod handler;

fn main() -> Result<(), String> {
    let args = Args::parse();

    match args.command {
        Command::Init { name, description } => {
            return handle_init(name, description);
        }
    }
}
