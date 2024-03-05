// src/ast.rs

mod display_impl;
pub use display_impl::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Number(i64),
    Variable(String),
    BinaryOp(Box<BinaryOp>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOp {
    pub kind: BinaryOpKind,
    pub left: Expr,
    pub right: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Assignment(Assignment),
    If(IfStmt),
    While(WhileStmt),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    pub name: String,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfStmt {
    pub guard: Expr,
    pub true_branch: Vec<Stmt>,
    pub false_branch: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileStmt {
    pub guard: Expr,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhiNode {
    pub variable: String,
    pub incoming: Vec<(String, Expr)>, // (block label, value)
}

// AST macros

#[macro_export]
macro_rules! stmt {
    ((if ($g:tt) $tt:tt else $ff:tt)) => {
        Stmt::If(IfStmt {
            guard: expr!($g),
            true_branch: block!($tt),
            false_branch: block!($ff),
        })
    };
    (($i:ident = $e:tt)) => {
        Stmt::Assignment(Assignment {
            name: ($i).to_string(),
            value: expr!($e),
        })
    };
    ((while ($g:tt) $body:tt)) => {
        Stmt::While(WhileStmt {
            guard: expr!($g),
            body: block!($body),
        })
    };
}

#[macro_export]
macro_rules! block {
    ({$($s:tt);*}) => {
        vec![$(stmt!($s)),*]
    };
}

#[macro_export]
macro_rules! expr {
    (($l:tt $op:tt $r:tt)) => {
        Expr::BinaryOp(Box::new(BinaryOp {
            kind: op!($op),
            left: expr!($l),
            right: expr!($r),
        }))
    };
    ($n:literal) => {
        Expr::Number($n)
    };
    ($x:ident) => {
        Expr::Variable($x.to_string())
    };
}

#[macro_export]
macro_rules! op {
    (+) => {
        BinaryOpKind::Add
    };
    (-) => {
        BinaryOpKind::Subtract
    };
    (*) => {
        BinaryOpKind::Multiply
    };
    (/) => {
        BinaryOpKind::Divide
    };
    (>) => {
        BinaryOpKind::GreaterThan
    };
    (<) => {
        BinaryOpKind::LessThan
    };
    (>=) => {
        BinaryOpKind::GreaterThanOrEqual
    };
    (<=) => {
        BinaryOpKind::LessThanOrEqual
    };
    (==) => {
        BinaryOpKind::Equal
    };
}
