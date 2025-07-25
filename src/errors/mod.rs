use thiserror::Error;
#[derive(Error, Debug)]
pub enum LexErr { #[error("bad char")] Bad }