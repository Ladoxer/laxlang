#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    // keywords
    Relax, Flow, Catch, Guard, Use,
    // decorators
    AtGuard,          // '@guard.'
    // operators
    FlowBind,         // '~>'
    // punctuation
    LBrace, RBrace, LParen, RParen,
    Colon, Semicolon, Comma,
    // literals/ids
    Ident(String), Str(String), Num(f64),
    // misc
    Eof,
}

pub fn tokenize(code: &str) -> Result<Vec<Token>, String> {
    // super-minimal starter: splits by whitespace & single-char symbols
    // Replace later with a real scanner.
    Ok(vec![])
}

logos::logos! {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Tok {
        #[token("relax")] Relax,
        #[token("flow")]  Flow,
        #[token("catch")] Catch,
        #[token("@guard.")] AtGuard,
        #[token("~>")] FlowBind,
        #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())] Ident(String),
        #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())] Str(String),
        #[regex(r"[0-9]+(\.[0-9]+)?", |lex| lex.slice().parse())] Num(f64),
        #[error] #[regex(r"[ \t\n\r]+", logos::skip)] Error,
    }
}