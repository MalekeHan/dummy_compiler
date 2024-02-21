mod ast;
use crate::ast::{Expr, BinaryOp, BinaryOpKind, Assignment, If};

/*
SAMPLE PROGRAM:
x = 10
y = 20
if (x > y) {
  x = x + 1
} else {
  y = y + 1
}

*/

fn main(){
    // create the assignments here
    let assign_x = Expr::Assignment(Box::new(Assignment {
        name: "x".to_string(),
        value: Expr::Number(10.0),
    }));

    let assign_y = Expr::Assignment(Box::new(Assignment {
        name: "y".to_string(),
        value: Expr::Number(20.0),
    }));

    // create the condition for the if statement (x > y)
    let condition = Expr::BinaryOp(Box::new(BinaryOp {
        kind: BinaryOpKind::GreaterThan,
        left: Expr::Variable("x".to_string()),
        right: Expr::Variable("y".to_string()),
    }));

    // create the bodies for the if and else branches
    let if_body = vec![Expr::Assignment(Box::new(Assignment {
        name: "x".to_string(),
        value: Expr::BinaryOp(Box::new(BinaryOp {
            kind: BinaryOpKind::Add,
            left: Expr::Variable("x".to_string()),
            right: Expr::Number(1.0),
        })),
    }))];

    // create the else bodty
    let else_body = vec![Expr::Assignment(Box::new(Assignment {
        name: "y".to_string(),
        value: Expr::BinaryOp(Box::new(BinaryOp {
            kind: BinaryOpKind::Add,
            left: Expr::Variable("y".to_string()),
            right: Expr::Number(1.0),
        })),
    }))];

    // create the whole entire if statement
    let if_statement = Expr::If(Box::new(If {
        condition,
        consequence: if_body,
        alternative: else_body,
    }));

    // the complete program
    let program = vec![assign_x, assign_y, if_statement];

}
