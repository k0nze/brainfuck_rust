use custom_error::custom_error;
use std::cmp::Ordering;
use std::error::Error;
use std::io::Read;

use crate::token::TokenValue;
use crate::State;
use crate::Token;

custom_error! { pub InterpreterError
    TokenIndexOutOfBound = "token index is out of bound",
    NestingError = "loop nesting is incorrect"
}

pub struct Interpreter<'a> {
    pub state: State,
    pub tokens: &'a Vec<Token>,
    pub token_index: usize,
}

impl<'a> Interpreter<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Interpreter<'a> {
        let state = State::new();
        Interpreter {
            state,
            tokens,
            token_index: 0,
        }
    }

    /// Interprets the token stream
    pub fn interpret(&mut self) -> Result<(), Box<dyn Error>> {
        let mut token = &self.tokens[self.token_index];

        while token.value != TokenValue::End {
            match token.value {
                TokenValue::MoveRight => {
                    self.state.move_right()?;
                    self.token_index += 1;
                }
                TokenValue::MoveLeft => {
                    self.state.move_left()?;
                    self.token_index += 1;
                }
                TokenValue::IncrementCell => {
                    self.state.increment_cell()?;
                    self.token_index += 1;
                }
                TokenValue::DecrementCell => {
                    self.state.decrement_cell()?;
                    self.token_index += 1;
                }
                TokenValue::Output => {
                    print!("{}", self.state.get_cell_value() as char);
                    self.token_index += 1;
                }
                TokenValue::Input => {
                    // TODO pass a different input stream to be able to read from a file or a pipe
                    let input = std::io::stdin()
                        .bytes()
                        .next()
                        .and_then(|result| result.ok())
                        .unwrap();
                    self.state.set_cell_value(input);
                    self.token_index += 1;
                }
                TokenValue::JumpForwardIfZero => match self.state.cells[self.state.pointer] {
                    0 => {
                        self.jump_forward()?;
                    }
                    _ => {
                        self.token_index += 1;
                    }
                },
                TokenValue::JumpBackwardIfNonZero => match self.state.cells[self.state.pointer] {
                    0 => {
                        self.token_index += 1;
                    }
                    _ => {
                        self.jump_backward()?;
                    }
                },
                _ => {}
            }
            token = &self.tokens[self.token_index];
        }

        Ok(())
    }

    /// Jumps forward in the token stream from current token index to matching ]
    pub fn jump_forward(&mut self) -> Result<(), InterpreterError> {
        // move forward to find matching ]
        let mut nesting_counter = 0;
        let mut token_search_index = self.token_index + 1;

        loop {
            match self.tokens[token_search_index].value {
                TokenValue::JumpForwardIfZero => {
                    nesting_counter += 1;
                }
                TokenValue::JumpBackwardIfNonZero => {
                    // no nesting, therefore matching ] was found
                    match nesting_counter.cmp(&0) {
                        Ordering::Equal => {
                            self.token_index = token_search_index + 1;
                            break;
                        }
                        Ordering::Greater => nesting_counter -= 1,
                        Ordering::Less => return Err(InterpreterError::NestingError),
                    };
                }
                _ => {}
            }
            token_search_index += 1;

            // check if token_search_index is out of bounds
            if token_search_index > self.tokens.len() {
                return Err(InterpreterError::TokenIndexOutOfBound);
            }
        }

        Ok(())
    }

    /// Jumps backward in the token stream from current token index to matching [
    pub fn jump_backward(&mut self) -> Result<(), InterpreterError> {
        // move backward to find matching [
        let mut nesting_counter = 0;
        let mut token_search_index = self.token_index - 1;

        loop {
            match self.tokens[token_search_index].value {
                TokenValue::JumpBackwardIfNonZero => {
                    nesting_counter += 1;
                }
                TokenValue::JumpForwardIfZero => {
                    // no nesting, therefore matching [ was found
                    match nesting_counter.cmp(&0) {
                        Ordering::Equal => {
                            self.token_index = token_search_index + 1;
                            break;
                        }
                        Ordering::Greater => {
                            nesting_counter -= 1;
                        }
                        Ordering::Less => return Err(InterpreterError::NestingError),
                    }
                }
                _ => {}
            }
            token_search_index -= 1;

            // check if token_search_index is out of bounds
            if token_search_index > self.tokens.len() {
                return Err(InterpreterError::TokenIndexOutOfBound);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_program() {
        // program: ++++><-.
        let tokens = vec![
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('>').unwrap(),
            Token::build('<').unwrap(),
            Token::build('-').unwrap(),
            Token::build('.').unwrap(),
            Token::build_end(),
        ];

        let mut interpreter = Interpreter::new(&tokens);
        interpreter.interpret().unwrap();
        assert_eq!(interpreter.token_index, tokens.len() - 1);

        let state = interpreter.state;
        assert_eq!(state.pointer, 0);
        assert_eq!(state.cells[0], 3);
    }

    #[test]
    fn test_jump_forward_no_nesting() {
        // program: >[>>>]+>
        let tokens = vec![
            Token::build('>').unwrap(),
            Token::build('[').unwrap(),
            Token::build('>').unwrap(),
            Token::build('>').unwrap(),
            Token::build('>').unwrap(),
            Token::build(']').unwrap(),
            Token::build('+').unwrap(),
            Token::build('>').unwrap(),
            Token::build_end(),
        ];

        let mut interpreter = Interpreter::new(&tokens);
        interpreter.interpret().unwrap();
        assert_eq!(interpreter.token_index, tokens.len() - 1);

        let state = interpreter.state;
        assert_eq!(state.pointer, 2);
        assert_eq!(state.cells[1], 1);
    }

    #[test]
    fn test_jump_backward_no_nesting() {
        // program: ++[-]+>
        let tokens = vec![
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('[').unwrap(),
            Token::build('-').unwrap(),
            Token::build(']').unwrap(),
            Token::build('+').unwrap(),
            Token::build('>').unwrap(),
            Token::build_end(),
        ];

        let mut interpreter = Interpreter::new(&tokens);
        interpreter.interpret().unwrap();
        assert_eq!(interpreter.token_index, tokens.len() - 1);

        let state = interpreter.state;
        assert_eq!(state.pointer, 1);
        assert_eq!(state.cells[0], 1);
        assert_eq!(state.cells[1], 0);
    }

    #[test]
    fn test_loop_no_nesting() {
        // program: +_31_+[>+<-]>.
        // this programs sets cell 0 to 33 and than increments cell 1 in a loop while it decrements cell 0
        // lastly, cell 1 will be printed with now contains the first "visible" ASCII character '!' (33)
        let mut tokens = Vec::new();
        for _ in 0..33 {
            tokens.push(Token::build('+').unwrap());
        }
        tokens.push(Token::build('[').unwrap());
        tokens.push(Token::build('>').unwrap());
        tokens.push(Token::build('+').unwrap());
        tokens.push(Token::build('<').unwrap());
        tokens.push(Token::build('-').unwrap());
        tokens.push(Token::build(']').unwrap());
        tokens.push(Token::build('>').unwrap());
        tokens.push(Token::build('.').unwrap());
        tokens.push(Token::build_end());

        let mut interpreter = Interpreter::new(&tokens);
        interpreter.interpret().unwrap();
        assert_eq!(interpreter.token_index, tokens.len() - 1);

        let state = interpreter.state;
        assert_eq!(state.pointer, 1);
        assert_eq!(state.cells[0], 0);
        assert_eq!(state.cells[1], 33);
    }

    #[test]
    fn test_nested_loop() {
        // program: ++++++++>++++>++<<[->[->[->+<]<]<]
        let tokens = vec![
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('>').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('>').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('<').unwrap(),
            Token::build('<').unwrap(),
            Token::build('[').unwrap(),
            Token::build('-').unwrap(),
            Token::build('>').unwrap(),
            Token::build('[').unwrap(),
            Token::build('-').unwrap(),
            Token::build('>').unwrap(),
            Token::build('[').unwrap(),
            Token::build('-').unwrap(),
            Token::build('>').unwrap(),
            Token::build('+').unwrap(),
            Token::build('<').unwrap(),
            Token::build(']').unwrap(),
            Token::build('<').unwrap(),
            Token::build(']').unwrap(),
            Token::build('<').unwrap(),
            Token::build(']').unwrap(),
            Token::build_end(),
        ];

        let mut interpreter = Interpreter::new(&tokens);
        interpreter.interpret().unwrap();
        assert_eq!(interpreter.token_index, tokens.len() - 1);

        let state = interpreter.state;
        assert_eq!(state.pointer, 0);
        assert_eq!(state.cells[0], 0);
        assert_eq!(state.cells[1], 0);
        assert_eq!(state.cells[2], 0);
        assert_eq!(state.cells[3], 2);
    }

    #[test]
    fn test_closing_nesting_error() {
        // program: ++++++++>++++>++<<[->[->[->+<<]<]
        let tokens = vec![
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('>').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('>').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('<').unwrap(),
            Token::build('<').unwrap(),
            Token::build('[').unwrap(),
            Token::build('-').unwrap(),
            Token::build('>').unwrap(),
            Token::build('[').unwrap(),
            Token::build('-').unwrap(),
            Token::build('>').unwrap(),
            Token::build('[').unwrap(),
            Token::build('-').unwrap(),
            Token::build('>').unwrap(),
            Token::build('+').unwrap(),
            Token::build('<').unwrap(),
            Token::build('<').unwrap(),
            Token::build(']').unwrap(),
            Token::build('<').unwrap(),
            Token::build(']').unwrap(),
            Token::build_end(),
        ];

        let mut interpreter = Interpreter::new(&tokens);
        assert!(interpreter.interpret().is_err());
    }

    #[test]
    fn test_opening_nesting_error() {
        // program: ++++++++>++++>++<<[->->[->+<]<]<]
        let tokens = vec![
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('>').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('>').unwrap(),
            Token::build('+').unwrap(),
            Token::build('+').unwrap(),
            Token::build('<').unwrap(),
            Token::build('<').unwrap(),
            Token::build('[').unwrap(),
            Token::build('-').unwrap(),
            Token::build('>').unwrap(),
            Token::build('-').unwrap(),
            Token::build('>').unwrap(),
            Token::build('[').unwrap(),
            Token::build('-').unwrap(),
            Token::build('>').unwrap(),
            Token::build('+').unwrap(),
            Token::build('<').unwrap(),
            Token::build(']').unwrap(),
            Token::build('<').unwrap(),
            Token::build(']').unwrap(),
            Token::build('<').unwrap(),
            Token::build(']').unwrap(),
            Token::build_end(),
        ];

        let mut interpreter = Interpreter::new(&tokens);
        assert!(interpreter.interpret().is_err());
    }
}
