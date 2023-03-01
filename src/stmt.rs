use crate::Expr;
#[derive(Clone)]
pub enum Stmt {
    Break,
    Empty,
    ExprStmt(Expr),
    Assignment(Expr, Expr),
    LocalAssignment(Expr, Expr),
    Block(Vec<Stmt>),
    DoBlock(Vec<Stmt>),
    IfStmt(Expr, /* conditional */ Box<Stmt> /* body */, Box<Stmt> /* else stmts */),
    WhileLoop(Expr, /* conditional */ Box<Stmt> /* body */),
    RepeatUntilLoop(Box<Stmt>, Expr),
    Return(Expr),
    // Implementation detail, not visible to users
    Chunk(Vec<Stmt>),
}