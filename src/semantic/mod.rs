pub struct TypeChecker { /* ... */ }
impl TypeChecker {
    pub fn check(&self, ast: &AST) -> Result<(), Error> {
        // Traverse AST, infer types,
        // Enforce Stack Guards, Flow Chain correctness, etc.
    }
}