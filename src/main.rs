mod logger;

use clap::{Parser, Subcommand};
use log::debug;
use std::fs;
use std::io;
use vkmsctl::VkmsDeviceBuilder;

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

    /// Remove a VKMS device.
    Remove {
        /// Name of the VKMS device to remove.
        name: String,
    },
}

/// Creates a VKMS device from the given JSON file.
///
/// # Errors
///
/// Returns an error if the JSON file is invalid or the VKMS device cannot be built.
pub fn create_vkms_device(configfs_path: &str, json_path: &str) -> Result<(), io::Error> {
    let builder = VkmsDeviceBuilder::from_json(configfs_path, json_path)?;
    builder.build()?;

    Ok(())
}

/// List all VKMS devices in the given configfs path.
///
/// # Errors
///
/// Returns an error if there is a problem reading the device from the filesystem.
fn list_vkms_devices(configfs_path: &str) -> Result<(), io::Error> {
    let mut devices = Vec::new();

    let device_dirs = fs::read_dir(format!("{configfs_path}/vkms"))?;
    for device_dir in device_dirs {
        let name = device_dir?.file_name().to_string_lossy().into_owned();
        let device = VkmsDeviceBuilder::from_fs(configfs_path, &name)?;
        devices.push(device);
    }

    for device in devices {
        println!("{device:?}");
    }

    Ok(())
}

/// Remove a VKMS device in the given configfs path.
///
/// # Errors
///
/// Returns an error if there is a problem deleting the device from the filesystem.
fn remove_vkms_device(configfs_path: &str, name: &str) -> Result<(), io::Error> {
    let device = VkmsDeviceBuilder::from_fs(configfs_path, name)?;
    device.remove()
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    logger::init(args.verbose).expect("Error initializing logger, was logger::init called twice?");

    debug!("Command line args: {args:?}");

    let configfs_path = args.configfs_path;

    match args.command {
        Some(Commands::Create { path }) => create_vkms_device(&configfs_path, &path),
        Some(Commands::List {}) => list_vkms_devices(&configfs_path),
        Some(Commands::Remove { name }) => remove_vkms_device(&configfs_path, &name),
        None => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Unknown command provided",
        )),
    }
}
