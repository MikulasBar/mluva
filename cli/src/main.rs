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
        },

        Commands::Run => {
            if let Err(e) = commands::run::command() {
                eprintln!("Error during run: {}", e);
            }
        },

        Commands::Uninit => {
            if let Err(e) = commands::uninit::command() {
                eprintln!("Error during uninit: {}", e);
            }
        },

        Commands::Build => {
            if let Err(e) = commands::build::command() {
                eprintln!("Error during build: {}", e);
            }
        },
    }
}





