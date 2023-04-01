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

/// Runs a brainfuck program
///
/// # Arguments
///
/// * `config` - Config containing the file path to the brainfuck program
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read brainfuck file into string
    let program_string = fs::read_to_string(config.brainfuck_file_path)?;
    let tokens = lex(&program_string);

    interpret(&tokens);

    Ok(())
}

/// Returns a vector of Token from a string
///
/// # Arguments
///
/// * `program_string` - A string containing the brainfuck program
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

#[cfg(test)]
mod tests {
    use super::*;
    use token::{Token, TokenValue};

    #[test]
    fn lexer_test() {
        let program_string = "\
<>+-adsfl ageaf
.,[qowejga]";

        let tokens = lex(program_string);

        assert_eq!(
            vec![
                Token {
                    value: TokenValue::MoveLeft
                },
                Token {
                    value: TokenValue::MoveRight
                },
                Token {
                    value: TokenValue::IncrementCell
                },
                Token {
                    value: TokenValue::DecrementCell
                },
                Token {
                    value: TokenValue::Output
                },
                Token {
                    value: TokenValue::Input
                },
                Token {
                    value: TokenValue::JumpForwardIfZero
                },
                Token {
                    value: TokenValue::JumpBackwardIfNonZero
                }
            ],
            tokens
        );
    }
}
