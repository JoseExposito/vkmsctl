mod args_parser;
mod create;
mod logger;

use log::debug;
use std::io;

fn main() -> Result<(), io::Error> {
    let args = args_parser::parse();
    logger::init(args.verbose).expect("Error initializing logger, was logger::init called twice?");

    debug!("Command line args: {args:?}");

    let configfs_path = args.configfs_path;

    match args.command {
        Some(args_parser::Commands::Create { path }) => {
            create::create_vkms_device(&configfs_path, &path)
        }
        None => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Unknown command provided",
        )),
    }
}
