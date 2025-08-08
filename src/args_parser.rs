use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// If set, more verbose logging will be used.
    #[arg(short, long)]
    pub verbose: bool,

    /// Directory were configfs is mounted.
    #[arg(long, default_value = "/sys/kernel/config")]
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
