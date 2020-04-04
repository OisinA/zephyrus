#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    Illegal,
    EOF,

    Identifier(String),
    Str(String),

    Equals,
    Comma,

    Quote,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    None,
}