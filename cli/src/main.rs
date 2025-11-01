use clap::Parser;

use crate::{cli::Cli, commands::Commands};
mod cli;
mod commands;
mod config;
mod module_metadata;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            if let Err(e) = commands::init::command() {
                eprintln!("Error during init: {}", e);
            }
        }

        Commands::Run => {
            let res = commands::run::command();
            if let Err(e) = res {
                eprintln!("{}", e);
            }
        }

        Commands::Uninit => {
            if let Err(e) = commands::uninit::command() {
                eprintln!("Error during uninit: {}", e);
            }
        }

        Commands::Build => {
            let res = commands::build::command();
            if let Err(e) = res {
                eprintln!("{}", e);
            }
        }
    }
}
