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
    /// Create a new VKMS device
    Create {
        /// Path to the JSON file describing the VKMS device.
        path: String,
    },
}

pub fn parse() -> Args {
    Args::parse()
}
