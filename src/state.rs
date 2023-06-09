use custom_error::custom_error;

custom_error! { pub StateTransitionError
    PointerUnderflow = "pointer underflow (state.pointer < 0)",
    PointerOverflow = "pointer overflow (state.pointer > 29999)",
    CellUnderflow = "cell underflow (state.cells[*] < 0)",
    CellOverflow = "cell underflow (state.cells[*] > 255)",
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

    pub fn increment_cell(&mut self) -> Result<(), StateTransitionError> {
        match self.cells[self.pointer].checked_add(1) {
            None => Err(StateTransitionError::CellOverflow),
            Some(pointer_value) => {
                self.cells[self.pointer] = pointer_value;
                Ok(())
            }
        }
    }

    pub fn decrement_cell(&mut self) -> Result<(), StateTransitionError> {
        match self.cells[self.pointer].checked_sub(1) {
            None => Err(StateTransitionError::CellUnderflow),
            Some(pointer_value) => {
                self.cells[self.pointer] = pointer_value;
                Ok(())
            }
        }
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
                "moving right from cell 29999 should lead to an PointerOverflow"
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
        state.increment_cell().unwrap();
        let cell_value = state.get_cell_value();

        assert_eq!(cell_value, 43);

        for _ in 0..212 {
            state.increment_cell().unwrap();
        }

        match state.increment_cell() {
            Err(StateTransitionError::CellOverflow) => assert!(true),
            _ => assert!(
                false,
                "incrementing a cell above 255 should cause a CellOverflow"
            ),
        }
    }

    #[test]
    fn test_decrement_cell_value() {
        let mut state = State::new();

        state.set_cell_value(42);
        state.decrement_cell().unwrap();
        state.move_right().unwrap();
        state.move_left().unwrap();
        let cell_value = state.get_cell_value();

        assert_eq!(cell_value, 41);

        for _ in 0..41 {
            state.decrement_cell().unwrap();
        }

        match state.decrement_cell() {
            Err(StateTransitionError::CellUnderflow) => assert!(true),
            _ => assert!(
                false,
                "incrementing a cell below 0 should cause a CellUnderflow"
            ),
        }
    }
}
