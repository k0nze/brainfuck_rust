use custom_error::custom_error;

custom_error! { pub StateTransitionError
    PointerUnderflow = "pointer underflow (state.pointer < 0)",
    PointerOverflow = "pointer overflow (state.pointer > 29999)",
}

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
