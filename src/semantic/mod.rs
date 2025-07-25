pub struct Checker { scopes: Vec<HashMap<String, Type>>, errors: Vec<LangErr> }

impl Checker {
    pub fn infer_relax(&mut self, f: &Decl) { /* bidirectional algorithm */ }

    pub fn desugar_flow(&mut self, fc: &Expr) -> Option<TypedExpr> {
        // result is async function body (Vec<Stmt>)
        // ensure every Bind's variable used later exists
    }

    pub fn enforce_guards(&mut self, sg: &StackGuard) {
        // verify guard combination legal (e.g. rateLimit before authenticate)
    }

    pub fn smart_import(&mut self, u: &UseStmt) {
        // record dependency for later codegen
    }

    pub fn hint_perf(&self, e: &Expr) {
        if let Expr::ForLoop{ body, .. } = e {
            if self.detect_n1(body) {
               self.warnings.push(PerfHint::NPlus1(e.span()));
            }
        }
    }
}