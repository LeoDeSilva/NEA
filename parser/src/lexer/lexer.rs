use std::{iter::Peekable, str::Chars};
use super::token::{Token, SyntaxError};


pub struct Lexer<'a> {
    program: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(program: &'a str) -> Self {
        Lexer {
            program: program.chars().peekable(),
        }
    }

    pub fn read_char(&mut self) -> Option<char> {
        self.program.next()
    }

    pub fn peek_char(&mut self) -> Option<&char> {
        self.program.peek()
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, SyntaxError> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(ch) = self.read_char() {
            match ch {
                ch if ch.is_whitespace() => {}
                '(' => tokens.push(Token::LPAREN),
                ')' => tokens.push(Token::RPAREN),

                '+' => tokens.push(Token::ADD),
                '-' => tokens.push(Token::SUB),
                '*' => tokens.push(Token::MUL),
                '/' => tokens.push(Token::DIV),

                '0'..='9' => {
                    let mut numeral = String::new();
                    while let Some(ch) = self.peek_char() {
                        if !ch.is_numeric() {
                            break;
                        }

                        numeral.push(self.read_char().unwrap());
                    }

                    tokens.push(Token::Number(
                        (ch.to_string() + &numeral).parse::<u32>().unwrap(),
                    ));
                }

                _ => {
                    return Err(SyntaxError::new(format!(
                        "SyntaxError: invalid character in source code '{}'",
                        ch
                    )))
                }
            }
        }

        tokens.push(Token::EOF);
        Ok(tokens)
    }
}
