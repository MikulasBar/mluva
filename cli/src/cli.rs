use clap::Parser;

use crate::commands::Commands;

#[derive(Parser)]
#[command(name = "mluva")]
#[command(about = "Mluva language compiler and interpreter")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
