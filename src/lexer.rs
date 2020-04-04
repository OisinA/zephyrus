use std::iter::Peekable;
use std::str::Chars;
use crate::token::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {

    pub fn new(input: &'a String) -> Lexer<'_> {
        Lexer{input: input.chars().peekable()}
    }

    fn clear_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.input.next();
                continue;
            }
            break;
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.clear_whitespace();
        match self.input.next() {
            Some('#') =>  {
                self.skip_line();
                Token::None
            },
            Some('=') => Token::Equals,
            Some(',') => Token::Comma,
            Some('"') => {
                Token::Str(self.read_string())
            },
            Some('(') => Token::LBracket,
            Some(')') => Token::RBracket,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some(ch) => {
                if ch.is_alphanumeric() || ch == '/' || ch == ':' || ch == '_' || ch == '.' || ch.is_numeric() {
                    let ident = self.read_identifier(ch);
                    return Token::Identifier(ident)
                }
                return Token::Illegal
            }
            None => Token::EOF,
            _ => Token::Illegal,
        }
    }

    pub fn skip_line(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch == '\n' {
                break
            }
            self.input.next();
        }
    }

    pub fn read_identifier(&mut self, ch: char) -> String {
        let mut identifier = String::new();
        identifier.push(ch);

        while let Some(&ch) = self.input.peek() {
            if ch.is_alphanumeric() || ch == '/' || ch == ':' || ch == '_' || ch == '.' || ch.is_numeric() {
                identifier.push(self.input.next().unwrap());
                continue;
            }
            break;
        }

        identifier
    }

    pub fn read_string(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(&ch) = self.input.peek() {
            if ch != '"' {
                identifier.push(self.input.next().unwrap());
                continue;
            }
            break;
        }
        self.input.next();
        identifier
    }

}