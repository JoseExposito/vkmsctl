use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory were ConfigFS is mounted/needs to be mounted.
    #[arg(long, default_value = "/config")]
    pub configfs_path: String,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Display current configuration.
    Config {},
}

pub fn parse() -> Args {
    Args::parse()
}
