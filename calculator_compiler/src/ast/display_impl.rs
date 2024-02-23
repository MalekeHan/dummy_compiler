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
                    BinaryOpKind::Add => todo!(),
                    BinaryOpKind::Subtract => todo!(),
                    BinaryOpKind::Multiply => todo!(),
                    BinaryOpKind::Divide => todo!(),
                    BinaryOpKind::GreaterThan => todo!(),
                    BinaryOpKind::LessThan => todo!(),
                    BinaryOpKind::GreaterThanOrEqual => todo!(),
                    BinaryOpKind::LessThanOrEqual => todo!(),
                    BinaryOpKind::Equal => todo!(),
                }
                write!(f, "{})", op.right)
            }
        }
    }
}
