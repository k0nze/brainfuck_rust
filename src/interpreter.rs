use crate::token::TokenValue;
use crate::State;
use crate::Token;

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

    pub fn interpret(&mut self) {
        let mut token = &self.tokens[self.token_index];

        while token.value != TokenValue::End {
            match token.value {
                TokenValue::MoveRight => {
                    self.state.move_right().unwrap();
                    self.token_index += 1;
                }
                TokenValue::MoveLeft => {
                    self.state.move_left().unwrap();
                    self.token_index += 1;
                }
                TokenValue::IncrementCell => {
                    self.state.increment_cell();
                    self.token_index += 1;
                }
                TokenValue::DecrementCell => {
                    self.state.decrement_cell();
                    self.token_index += 1;
                }
                TokenValue::Output => {
                    print!("{}", self.state.get_cell_value() as char);
                    self.token_index += 1;
                }
                TokenValue::Input => {
                    self.state.set_cell_value(42);
                    self.token_index += 1;
                }
                TokenValue::JumpForwardIfZero => {
                    println!("jump forward");
                }
                TokenValue::JumpBackwardIfNonZero => {
                    println!("jump backward");
                }
                _ => {}
            }
            token = &self.tokens[self.token_index];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_program() {
        // program: ++++><-.
        let tokens = vec![
            Token {
                value: TokenValue::IncrementCell,
            },
            Token {
                value: TokenValue::IncrementCell,
            },
            Token {
                value: TokenValue::IncrementCell,
            },
            Token {
                value: TokenValue::IncrementCell,
            },
            Token {
                value: TokenValue::MoveRight,
            },
            Token {
                value: TokenValue::MoveLeft,
            },
            Token {
                value: TokenValue::DecrementCell,
            },
            Token {
                value: TokenValue::Output,
            },
            Token {
                value: TokenValue::End,
            },
        ];

        let mut interpreter = Interpreter::new(&tokens);
        interpreter.interpret();

        let state = interpreter.state;
        assert_eq!(state.cells[0], 3);
    }
    /*
        #[test]
        fn test_jump_forward() {
            let program_string = "\
    +++ // increment cell 0 to 3
    >   // move pointer to cell 1
    ++  // increment cell 1 to 2
    [   // jump forward if current cell (1) is 0
    -   // decrement current cell (1)
    ]   // jump backward if current cell (1) is not 0
    ";
            println!("{}", program_string);
            assert!(true);
        }
    */
}
