#[macro_use]
extern crate log;

use clap::Parser;

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

    let x = 'ff'; println!("Hello, World!");
}
