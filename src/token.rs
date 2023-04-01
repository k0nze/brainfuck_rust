pub enum TokenValue {
    MoveRight,
    MoveLeft,
    IncrementCell,
    DecrementCell,
    Output,
    Input,
    JumpForwardIfZero,
    JumpBackwardIfNonZero,
}

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
