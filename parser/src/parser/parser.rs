use super::ast::{Expression, Literal, Operation, Prefix};
use crate::{lexer::token::SyntaxError, Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    tok: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let tok = tokens[0].clone();
        Parser {
            tokens: tokens,
            pos: 0,
            tok: tok,
        }
    }

    fn advance(&mut self) {
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
            self.tok = self.tokens[self.pos].clone();
        }
    }

    fn retreat(&mut self) {
        self.pos -= 1;
        self.tok = self.tokens[self.pos].clone();
    }

    fn assert(&self, t: Token) -> Result<(), SyntaxError> {
        if self.tok != t {
            return Err(SyntaxError::new(format!(
                "SyntaxError: expected to encounter token of type '{:?}', instead encountered '{:?}'",
                t,
                self.tok,
            )))
        }
        Ok(())
    }

    pub fn parse(&mut self) -> Result<Expression, SyntaxError> {
        return self.parse_expression(0);
    }

    fn parse_expression(&mut self, rbp: i32) -> Result<Expression, SyntaxError> {
        let mut lhs = match self.parse_atom() {
            Ok(lhs) => lhs,
            Err(e) => {
                return Err(e);
            }
        };

        self.advance();
        let mut peek_rbp = self.tok.get_precedence();

        while self.pos < self.tokens.len() && peek_rbp > rbp {
            lhs = match self.parse_infix(lhs, self.tok.clone()) {
                Ok(lhs) => lhs,
                Err(e) => {
                    return Err(e);
                }
            };

            peek_rbp = self.tok.get_precedence();
        }

        Ok(lhs)
    }

    fn parse_infix(&mut self, lhs: Expression, op: Token) -> Result<Expression, SyntaxError> {
        self.advance();
        let rhs = match self.parse_expression(op.get_precedence() + 1) {
            Ok(rhs) => rhs,
            Err(e) => return Err(e),
        };

        Ok(Expression::InfixExpr(
            Box::new(lhs),
            Operation::try_from(op).unwrap(),
            Box::new(rhs),
        ))
    }

    fn parse_atom(&mut self) -> Result<Expression, SyntaxError> {
        let expr = match self.tok {
            Token::Number(n) => Expression::LiteralExpr(Literal::Number(n as i32)),

            Token::SUB => {
                self.advance();
                let expr = match self.parse_expression(40) {
                    Ok(expr) => expr,
                    Err(e) => return Err(e),
                };

                self.retreat();
                Expression::PrefixExpr(Prefix::Minus, Box::new(expr))
            }

            Token::LPAREN => {
                self.advance();
                let expr = match self.parse_expression(0) {
                    Ok(expr) => expr,
                    Err(e) => return Err(e),
                };

                match self.assert(Token::RPAREN) {
                    Ok(_) => {},
                    Err(e) => return Err(e),
                };

                expr
            }

            _ => return Err(SyntaxError::new(
                format!(
                    "SyntaxError: unexpected token enocountered when parsing infix expression '{:?}'",
                    self.tok,
                )
            )),
        };

        Ok(expr)
    }
}
