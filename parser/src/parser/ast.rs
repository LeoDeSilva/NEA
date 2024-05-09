use crate::lexer::token::Token;

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    LiteralExpr(Literal),
    PrefixExpr(Prefix, Box<Expression>),
    InfixExpr(Box<Expression>, Operation, Box<Expression>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Number(i32),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Prefix {
    Minus,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl TryFrom<Token> for Operation {
    type Error = &'static str;
    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::ADD => Ok(Operation::Add),
            Token::SUB => Ok(Operation::Subtract),
            Token::MUL => Ok(Operation::Multiply),
            Token::DIV => Ok(Operation::Divide),
            _ => Err("Invalid Type: can only convert operators")
        } 
    }
}