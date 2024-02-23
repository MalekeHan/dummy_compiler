// src/ast.rs

mod display_impl;
pub use display_impl::*;

// expression in the calc app
pub enum Expr {
    Number(f64),
    Variable(String),
    BinaryOp(Box<BinaryOp>),
}

// binop
pub enum BinaryOpKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Equal,
}

pub struct BinaryOp {
    pub kind: BinaryOpKind,
    pub left: Expr,
    pub right: Expr,
}

pub enum Stmt {
    Assignment(Assignment),
    If(IfStmt),
    While(WhileStmt),
}

// reps an assignment (x =5 )
pub struct Assignment {
    pub name: String,
    pub value: Expr,
}

// reps an if
pub struct IfStmt {
    pub guard: Expr,
    pub true_branch: Vec<Stmt>,
    pub false_branch: Vec<Stmt>,
}

// reps a while
pub struct WhileStmt {
    pub guard: Expr,
    pub body: Vec<Stmt>,
}
// reps the phi node
pub struct PhiNode {
    pub variable: String,
    pub incoming: Vec<(String, Expr)>, // (block label, value)
}
