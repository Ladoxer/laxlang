#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Declaration(Declaration),
    Block(Vec<Statement>),
}

#[derive(Debug, Clone)]
pub enum Declaration {
    RelaxFunction { name: String, params: Vec<String>, body: Vec<Statement> },
    Function { name: String, params: Vec<String>, ret_type: String, body: Vec<Statement> },
    // ...add more as you create features
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    Call { callee: Box<Expression>, args: Vec<Expression> },
    FlowChain(FlowChain),
    StackGuard(StackGuard),
    // ...
}
