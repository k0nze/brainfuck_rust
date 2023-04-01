use std::error::Error;
use std::fs;

mod token;
use token::Token;

pub struct Config {
    pub brainfuck_file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("no path to brainfuck file provided.");
        }

        let brainfuck_file_path = args[1].clone();

        Ok(Config {
            brainfuck_file_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read brainfuck file into string
    let program_string = fs::read_to_string(config.brainfuck_file_path)?;
    let tokens = lex(&program_string);

    println!("{}", tokens.len());

    Ok(())
}

pub fn lex(program_string: &str) -> Vec<Token> {
    println!("{}", program_string);
    Vec::new()
}
