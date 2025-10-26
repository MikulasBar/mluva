use clap::Parser;

use crate::commands::Commands;


#[derive(Parser)]
#[command(name = "mluva")]
#[command(about = "Mluva language compiler and interpreter")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}