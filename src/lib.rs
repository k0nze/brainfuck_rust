use std::error::Error;
use std::fs;

mod token;
use token::Token;

// TODO move config into a separate file
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

    interpret(&tokens);

    Ok(())
}

pub fn lex(program_string: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for c in program_string.chars() {
        if let Some(t) = Token::build(c) {
            tokens.push(t);
        };
    }

    tokens
}

pub fn interpret(tokens: &Vec<Token>) {
    println!("{}", tokens.len());
}
