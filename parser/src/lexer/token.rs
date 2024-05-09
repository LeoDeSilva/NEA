#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token {
    Number(u32),

    LPAREN,
    RPAREN,

    ADD,
    SUB,
    MUL,
    DIV,
    EOF,
}

impl Token {
    pub fn get_precedence(&self) -> i32 {
        match self {
            Token::ADD | Token::SUB => 10,
            Token::MUL | Token::DIV => 20,
            _ => -1,
        }
    }
}

#[derive(Debug)]
pub struct SyntaxError {
    pub msg: String,
}

impl SyntaxError {
    pub fn new(msg: String) -> Self {
        SyntaxError { msg }
    }
}
