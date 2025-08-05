mod args_parser;

fn main() {
    let args = args_parser::parse();

    println!("Command line args: {args:?}");
}
