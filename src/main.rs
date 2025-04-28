mod args_parser;

fn display_current_config(_configfs_path: &str) {}

fn main() {
    let args = args_parser::parse();

    println!("Command line args: {args:?}");

    match &args.command {
        Some(args_parser::Commands::Config {}) => {
            display_current_config(&args.configfs_path);
        }
        None => {}
    }
}
