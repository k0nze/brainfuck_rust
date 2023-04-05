use std::error::Error;
use std::fs;

mod config;
mod interpreter;
mod state;
mod token;

pub use config::Config;
use interpreter::Interpreter;
use state::State;
use token::{Token, TokenValue};

/// Runs a brainfuck program
///
/// # Arguments
///
/// * `config` - Config containing the file path to the brainfuck program
pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    // read brainfuck file into string
    let program_string = fs::read_to_string(config.brainfuck_file_path)?;
    let tokens = lex(&program_string);

    interpret(&tokens)
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
    tokens.push(Token {
        value: TokenValue::End,
    });
    tokens
}

pub fn interpret(tokens: &Vec<Token>) -> Result<String, Box<dyn Error>> {
    let mut interpreter = Interpreter::new(tokens);
    interpreter.interpret()
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::{Token, TokenValue};

    #[test]
    fn test_lexer() {
        let program_string = "\
<>+-adsfl ageaf
.,[qowejga]";

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
                },
                Token {
                    value: TokenValue::End
                }
            ],
            lex(program_string)
        );
    }
}
