use clap::Subcommand;


#[derive(Subcommand)]
pub enum Commands {
    Init,
    Run,
}