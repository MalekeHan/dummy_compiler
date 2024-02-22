mod ast;
use crate::ast::{Expr, Stmt, BinaryOp, BinaryOpKind, Assignment, IfStmt, BinaryOp};

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

    let if_body = vec![
        Stmt::Assignment(Assignment {
            name: "x".to_string(),
            value: Expr::BinaryOp(Box::new(BinaryOp {
                kind: BinaryOpKind::Add,
                left: Expr::Variable("x".to_string()),
                right: Expr::Number(1.0),
            })),
        })
    ];

    let else_body = vec![
        Stmt::Assignment(Assignment {
            name: "y".to_string(),
            value: Expr::BinaryOp(Box::new(BinaryOp {
                kind: BinaryOpKind::Add,
                left: Expr::Variable("y".to_string()),
                right: Expr::Number(1.0),
            })),
        })
    ];

    // the if statement is now a statement with true and false branches
    let if_statement = Stmt::If(IfStmt {
        guard: condition,
        true_branch: if_body,
        false_branch: else_body,
    });

    // the complete program
    let program = vec![assign_x, assign_y, if_statement];

}
