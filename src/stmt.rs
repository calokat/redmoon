use crate::Expr;
pub enum Stmt {
    Empty,
    ExprStmt(Expr),
    Assignment(Expr, Expr),
    LocalAssignment(Expr, Expr),
    Block(Vec<Stmt>),
    IfStmt(Expr, /* conditional */ Box<Stmt> /* body */)
}