use custom_error::custom_error;

custom_error! { pub StateTransitionError
    PointerUnderflow = "pointer underflow (state.pointer < 0)",
    PointerOverflow = "pointer overflow (state.pointer > 29999)",
}

#[derive(Debug)]
pub struct State {
    pub cells: Vec<u8>,
    pub pointer: usize,
}

impl State {
    pub fn new() -> State {
        State {
            cells: vec![0; 30000],
            pointer: 0,
        }
    }

    pub fn move_left(&mut self) -> Result<(), StateTransitionError> {
        match self.pointer {
            0 => Err(StateTransitionError::PointerUnderflow),
            _ => {
                self.pointer -= 1;
                Ok(())
            }
        }
    }

    pub fn move_right(&mut self) -> Result<(), StateTransitionError> {
        match self.pointer {
            29999 => Err(StateTransitionError::PointerOverflow),
            _ => {
                self.pointer += 1;
                Ok(())
            }
        }
    }

    pub fn increment_cell(&mut self) {
        self.cells[self.pointer] += 1;
    }

    pub fn decrement_cell(&mut self) {
        self.cells[self.pointer] -= 1;
    }

    pub fn get_cell_value(&mut self) -> u8 {
        self.cells[self.pointer]
    }

    pub fn set_cell_value(&mut self, value: u8) {
        self.cells[self.pointer] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_left_underflow() {
        let mut state = State::new();

        match state.move_left() {
            Err(StateTransitionError::PointerUnderflow) => assert!(true),
            _ => assert!(
                false,
                "moving left from cell 0 should lead to an PointerUnderflowError"
            ),
        }
    }

    #[test]
    fn test_move_right_overflow() {
        let mut state = State::new();

        for _ in 0..29999 {
            state.move_right().unwrap();
        }

        match state.move_right() {
            Err(StateTransitionError::PointerOverflow) => assert!(true),
            _ => assert!(
                false,
                "moving right from cell 29999 should lead to an PointerOverflowError"
            ),
        }
    }

    #[test]
    fn test_set_get_cell_value() {
        let mut state = State::new();

        state.set_cell_value(42);
        state.move_right().unwrap();
        state.move_left().unwrap();
        let cell_value = state.get_cell_value();

        assert_eq!(cell_value, 42);
    }

    #[test]
    fn test_increment_cell_value() {
        let mut state = State::new();

        state.set_cell_value(42);
        state.move_right().unwrap();
        state.move_left().unwrap();
        state.increment_cell();
        let cell_value = state.get_cell_value();

        assert_eq!(cell_value, 43);
    }

    #[test]
    fn test_decrement_cell_value() {
        let mut state = State::new();

        state.set_cell_value(42);
        state.decrement_cell();
        state.move_right().unwrap();
        state.move_left().unwrap();
        let cell_value = state.get_cell_value();

        assert_eq!(cell_value, 41);
    }
}
