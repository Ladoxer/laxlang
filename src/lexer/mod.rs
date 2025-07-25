#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Number(f64),
    String(String),
    Relax, Flow, Catch, Guard, Use, Function,
    LParen, RParen, LBrace, RBrace, Semicolon, Comma, Colon,
    Plus, Minus, Star, Slash, Equal, EqualEqual,
    FlowBind, // ~>
    GuardApply, // @guard.
    Eof,
}

pub struct Lexer { /* fields omitted for brevity */ }
// Implement next_token, reading identifiers, numbers, symbols, etc.
