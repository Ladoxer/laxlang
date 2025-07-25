use crate::{lexer::Token, ast::*};

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    // placeholder: produce empty program
    Ok(Program(vec![]))
}