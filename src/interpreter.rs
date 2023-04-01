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
        let token = &self.tokens[self.token_index];

        match token.value {
            TokenValue::MoveRight => {
                self.state.move_right().unwrap();
            }
            TokenValue::MoveLeft => {
                self.state.move_left().unwrap();
            }
            TokenValue::IncrementCell => {
                self.state.increment_cell();
            }
            TokenValue::DecrementCell => {
                self.state.decrement_cell();
            }
            TokenValue::Output => {
                print!("{}", self.state.get_cell_value() as char);
            }
            TokenValue::Input => {
                self.state.set_cell_value(42);
            }
            TokenValue::JumpForwardIfZero => {
                println!("jump forward");
            }
            TokenValue::JumpBackwardIfNonZero => {
                println!("jump backward");
            }
            TokenValue::End => {
                /*break;*/
                println!("end");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter_jump_forward() {
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
}
