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
    /// Create a new VKMS device.
    Create {
        /// Path to the JSON file describing the VKMS device.
        path: String,
    },

    /// List all VKMS devices.
    ///
    /// Note that the output of this command is only a representation of the state of the VKMS
    /// device as it is in the filesystem.
    /// It doesn't include any state that is not stored in the filesystem.
    List {},
}

pub fn parse() -> Args {
    Args::parse()
}
