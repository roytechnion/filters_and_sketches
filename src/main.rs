//use std::env;
use std::process;
use clap::Parser;

use filters_and_sketches::Config;

fn main() {
    let config = Config::parse();

    if let Err(e) = filters_and_sketches::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
