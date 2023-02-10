use crate::Expr;
pub enum Stmt {
    Empty,
    ExprStmt(Expr),
    Assignment(Expr, Expr),
}