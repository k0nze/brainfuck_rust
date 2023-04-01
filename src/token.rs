use std::fmt;

#[derive(PartialEq, Eq)]
pub enum TokenValue {
    MoveRight,
    MoveLeft,
    IncrementCell,
    DecrementCell,
    Output,
    Input,
    JumpForwardIfZero,
    JumpBackwardIfNonZero,
    End,
}

impl fmt::Debug for TokenValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenValue::MoveRight => write!(f, "MoveRight"),
            TokenValue::MoveLeft => write!(f, "MoveLeft"),
            TokenValue::IncrementCell => write!(f, "IncrementCell"),
            TokenValue::DecrementCell => write!(f, "DecrementCell"),
            TokenValue::Output => write!(f, "Output"),
            TokenValue::Input => write!(f, "Input"),
            TokenValue::JumpForwardIfZero => write!(f, "JumpForwardIfZero"),
            TokenValue::JumpBackwardIfNonZero => write!(f, "JumpBackwardIfNonZero"),
            TokenValue::End => write!(f, "End"),
        }
    }
}

impl fmt::Display for TokenValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Eq)]
pub struct Token {
    pub value: TokenValue,
}

impl Token {
    pub fn build(c: char) -> Option<Token> {
        match c {
            '>' => Some(Token {
                value: TokenValue::MoveRight,
            }),
            '<' => Some(Token {
                value: TokenValue::MoveLeft,
            }),
            '+' => Some(Token {
                value: TokenValue::IncrementCell,
            }),
            '-' => Some(Token {
                value: TokenValue::DecrementCell,
            }),
            '.' => Some(Token {
                value: TokenValue::Output,
            }),
            ',' => Some(Token {
                value: TokenValue::Input,
            }),
            '[' => Some(Token {
                value: TokenValue::JumpForwardIfZero,
            }),
            ']' => Some(Token {
                value: TokenValue::JumpBackwardIfNonZero,
            }),
            _ => None,
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
