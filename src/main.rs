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
    simple_logger::init_with_level(log::Level::Trace);

    interpreter::test_from_main();

    //let args = Arguments::parse();

    //let mut source = std::fs::read_to_string(&args.input_file)
    //     .expect(&format!("Failed to open file: {}", args.input_file));

    //let mut interpreter = Interpreter::run(source);
}
