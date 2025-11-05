use clap::Parser;

use crate::{cli::Cli, commands::Commands};
mod cli;
mod commands;
mod config;
mod module_metadata;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init => commands::init::command(),
        Commands::Run => commands::run::command(),
        Commands::Build => commands::build::command().map(|_| ()),
    };

    if result.is_err() {
        std::process::exit(1);
    }
}
