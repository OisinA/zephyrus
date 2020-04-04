use crate::token::Token;
use crate::lexer::Lexer;
use std::error::Error;
use std::format;

pub struct Parser<'a> {
    lexer: Lexer<'a>,

    cur_token: Token,
    peek_token: Token,
}

impl Parser<'_> {

    pub fn new(mut lexer: Lexer<'_>) -> Parser<'_> {
        let mut lexer = lexer;
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser{lexer, cur_token, peek_token}
    }

    pub fn parse(&mut self) -> Result<String, &Error> {
        let mut output = String::new();
        while let s = self.cur_token.clone() {
            if s == Token::EOF {
                break;
            }
            output.push_str(&self.parse_statement(s));
            self.next_token()
        }

        Ok(output)
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_statement(&mut self, ident: Token) -> String {
        let mut output = String::new();
        match ident {
            Token::Identifier(s) => {
                let mut attrs = String::new();
                if self.peek_token == Token::LBracket {
                    self.next_token();
                    self.next_token();

                    loop {
                        let att = match self.cur_token.clone() {
                            Token::Identifier(s) => s,
                            _ => String::from("uhh"),
                        };
                        if self.peek_token != Token::Equals {
                            println!("Missing equals...");
                            break;
                        }
                        self.next_token();
                        self.next_token();
                        let val = match self.cur_token.clone() {
                            Token::Str(s) => s,
                            _ => String::from("uhh"),
                        };
                        attrs.push_str(&format!("{}=\"{}\" ", att, val));
                        if self.peek_token != Token::Comma {
                            break;
                        }
                        self.next_token();
                        self.next_token();
                    }
                    self.next_token();
                }

                if self.peek_token == Token::LBrace {
                    // Next token is a { so this must be a tag
                    output.push_str(&format!("<{} {}>", s, attrs));
                    self.next_token();
                    self.next_token();

                    if self.cur_token == Token::RBrace {
                        output.push_str(&format!("</{}>", s));
                        return output;
                    }

                    // Skip to the contents and parse each statement
                    while true {
                        match self.peek_token {
                            Token::Identifier(_) => {
                                output.push_str(&self.parse_statement(self.cur_token.clone()));
                                self.next_token();
                            },
                            Token::RBrace => {
                                output.push_str(&self.parse_statement(self.cur_token.clone()));
                                break;
                            }
                            Token::EOF => {
                                break;
                            }
                            _ => {
                                output.push_str(&self.parse_statement(self.cur_token.clone()));
                                self.next_token();
                            },
                        }
                    }
                    output.push_str(&format!("</{}>", s));
                } else {
                    output.push_str(&format!("<{} {} />", s, attrs));
                }
            },
            Token::Str(s) => {
                output.push_str(&s);
            },
            _ => ()
        }
        output
    }

}