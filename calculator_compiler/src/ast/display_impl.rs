use super::*;
use std::fmt::Display;

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Expr::*;

        match self {
            Number(n) => write!(f, "{n}"),
            Variable(x) => write!(f, "\"{x}\""),
            BinaryOp(op) => {
                write!(f, "({}", op.left)?;
                match op.kind {
                    BinaryOpKind::Add => write!(f, " + ")?,
                    BinaryOpKind::Subtract => write!(f, " - ")?,
                    BinaryOpKind::Multiply => write!(f, " * ")?,
                    BinaryOpKind::Divide => write!(f, " / ")?,
                    BinaryOpKind::GreaterThan => write!(f, " > ")?,
                    BinaryOpKind::LessThan => write!(f, " < ")?,
                    BinaryOpKind::GreaterThanOrEqual => write!(f, " >= ")?,
                    BinaryOpKind::LessThanOrEqual => write!(f, " <= ")?,
                    BinaryOpKind::Equal => write!(f, " == ")?,
                }
                write!(f, "{})", op.right)
            }
        }
    }
}
