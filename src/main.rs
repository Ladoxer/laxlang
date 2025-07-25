mod lexer;  mod parser;  mod ast;  mod semantic;  mod codegen;  mod errors;

use anyhow::Result;
use std::{fs, path::Path};

fn main() -> Result<()> {
    let src_path = std::env::args().nth(1).expect("give .lax file");
    let source   = fs::read_to_string(&src_path)?;
    let tokens   = lexer::tokenize(&source)?;
    let ast      = parser::parse(tokens)?;
    let typed    = semantic::analyze(&ast)?;
    let ts       = codegen::emit(&typed);
    let out_path = Path::new(&src_path).with_extension("ts");
    fs::write(out_path, ts)?;
    println!("✅  Generated TypeScript!");
    Ok(())
}