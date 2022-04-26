#[macro_use]
extern crate log;

mod interpreter;

use clap::Parser;
use interpreter::Interpreter;

#[derive(Debug, Parser)]
struct Arguments {
    #[clap()]
    input_file: String,

    #[clap(short, long)]
    output_file: Option<String>,
}

fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let args = Arguments::parse();

    let mut source = std::fs::read_to_string(args.input_file).expect("Failed to open file");

    let mut interpreter = Interpreter::run(source);
}
