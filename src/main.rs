mod args_parser;
mod create;
mod logger;

use log::debug;
use std::fs;
use std::io;
use vkmsctl::VkmsDeviceBuilder;

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

fn main() -> Result<(), io::Error> {
    let args = args_parser::parse();
    logger::init(args.verbose).expect("Error initializing logger, was logger::init called twice?");

    debug!("Command line args: {args:?}");

    let configfs_path = args.configfs_path;

    match args.command {
        Some(args_parser::Commands::Create { path }) => {
            create::create_vkms_device(&configfs_path, &path)
        }
        Some(args_parser::Commands::List {}) => list_vkms_devices(&configfs_path),
        None => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Unknown command provided",
        )),
    }
}
