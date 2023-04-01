use std::env;
use std::process;

use brainfuck_rust::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Interpreter error: {}", e);
        process::exit(1);
    }
}
