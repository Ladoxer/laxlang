#[derive(Clone, Debug)]
pub enum Decl {
    RelaxFn(String, Vec<Param>, Block),
    Fn(String, Vec<Param>, Type, Block),
}

#[derive(Clone, Debug)]
pub enum Expr {
    FlowChain(String, Vec<Param>, Vec<FlowOp>, Vec<Handler>),
    Call(Box<Expr>, Vec<Expr>),
    Ident(String),
    // …
}

#[derive(Clone, Debug)]
pub enum FlowOp {
    Bind(Box<Expr>, String),
    Map(Box<Expr>, Box<Expr>),
}