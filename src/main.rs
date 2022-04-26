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

    let mut interpreter = Interpreter::run();
}
