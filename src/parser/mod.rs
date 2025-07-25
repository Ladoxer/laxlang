pub struct Parser { /* fields omitted */ }
impl Parser {
    pub fn parse(&mut self) -> Result<Statement, ParseError> {
        // Read tokens and build AST
        // Example: detect "relax function ..." and build Declaration::RelaxFunction node
    }
}