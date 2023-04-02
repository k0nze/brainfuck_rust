use std::cmp::Ordering;

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
                TokenValue::JumpForwardIfZero => match self.state.cells[self.state.pointer] {
                    0 => {
                        self.jump_forward();
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
                        self.jump_backward();
                    }
                },
                _ => {}
            }
            token = &self.tokens[self.token_index];
        }
    }

    pub fn jump_forward(&mut self) {
        // move forward to find matching ]
        let mut nesting_counter = 0;
        let mut token_search_index = self.token_index + 1;

        // TODO handle out of bounds error
        loop {
            match self.tokens[token_search_index].value {
                TokenValue::JumpForwardIfZero => {
                    nesting_counter += 1;
                }
                TokenValue::JumpBackwardIfNonZero => {
                    // no nesting, therefore matching ] was found
                    // TODO nesting counter is less than 0 which means an something is wrong
                    match nesting_counter.cmp(&0) {
                        Ordering::Equal => {
                            self.token_index = token_search_index + 1;
                            break;
                        }
                        Ordering::Greater => nesting_counter -= 1,
                        Ordering::Less => (),
                    };
                }
                _ => {}
            }
            token_search_index += 1;
        }
    }

    pub fn jump_backward(&mut self) {
        // move backward to find matching [
        let mut nesting_counter = 0;
        let mut token_search_index = self.token_index - 1;

        // TODO handle out of bounds error
        loop {
            match self.tokens[token_search_index].value {
                TokenValue::JumpBackwardIfNonZero => {
                    nesting_counter += 1;
                }
                TokenValue::JumpForwardIfZero => {
                    // no nesting, therefore matching [ was found
                    // TODO nesting counter is less than 0 which means an something is wrong
                    match nesting_counter.cmp(&0) {
                        Ordering::Equal => {
                            self.token_index = token_search_index + 1;
                            break;
                        }
                        Ordering::Greater => {
                            nesting_counter -= 1;
                        }
                        Ordering::Less => (),
                    }
                }
                _ => {}
            }
            token_search_index -= 1;
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
        interpreter.interpret();
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
        interpreter.interpret();
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
        interpreter.interpret();
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
        interpreter.interpret();
        assert_eq!(interpreter.token_index, tokens.len() - 1);

        let state = interpreter.state;
        assert_eq!(state.pointer, 1);
        assert_eq!(state.cells[0], 0);
        assert_eq!(state.cells[1], 33);
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
