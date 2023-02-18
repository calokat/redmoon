use crate::Expr;
#[derive(Clone)]
pub enum Stmt {
    Empty,
    ExprStmt(Expr),
    Assignment(Expr, Expr),
    LocalAssignment(Expr, Expr),
    Block(Vec<Stmt>),
    IfStmt(Expr, /* conditional */ Box<Stmt> /* body */),
    WhileLoop(Expr, /* conditional */ Box<Stmt> /* body */),
    RepeatUntilLoop(Box<Stmt>, Expr),
}