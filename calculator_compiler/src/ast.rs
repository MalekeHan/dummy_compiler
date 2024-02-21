// src/ast.rs

// expression in the calc app
pub enum Expr {
    Number(f64),
    Variable(String),
    BinaryOp(Box<BinaryOp>),
    Assignment(Box<Assignment>),
    If(Box<If>),
    While(Box<While>),
    Phi(Vec<PhiNode>), // reps phi nodes for SSA form
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

// reps an assignment (x =5 )
pub struct Assignment {
    pub name: String,
    pub value: Expr,
}

// reps an if
pub struct If {
    pub guard: Expr,
    pub ttguard: Vec<Expr>,
    pub ffgaurd: Vec<Expr>,
}

// reps a while
pub struct While {
    pub guard: Expr,
    pub body: Vec<Expr>,
}

// reps the phi node 
pub struct PhiNode {
    pub variable: String,
    pub incoming: Vec<(String, Expr)>, // (block label, value)
}
