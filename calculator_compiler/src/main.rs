use ast::*;
use calculator_compiler::*;

fn main() {
    // Declare variable names as Rust variables
    let x = "foo";
    let y = "bar";
    let z = "baz";

    let s = stmt! {
        // While loop with a simple condition and nested if-else statements
        (while (x < 10) {
            // If statement with a binary operation in the guard
            (if ((x + 2) <= 10) {
                (x = (x + 1)); // Increment x
            } else {
                // Nested if without an else branch
                (if (y > 5) {
                    (y = (y - 1)); // Decrement y
                } else {
                })
            });
            // Assignment with a complex binary operation
            (z = ((x * 2) / (y + 3)));
        })
    };

    // More examples

    // Simple assignment
    let assignment_example = stmt! {
        (x = 42)
    };

    // If-else statement with binary operation guards and multiple branches
    let if_else_example = stmt! {
        (if ((x == y) && (z < 100)) {
            (x = 100);
            (y = 200);
        } else {
            (z = (x + y));
        })
    };

    // Nested while loop
    let nested_while_example = stmt! {
        (while (x < 50) {
            (while (y < x) {
                (y = (y + 1));
            });
            (x = (x + 10));
        })
    };

    // Print the constructed AST nodes
    println!("While with nested if-else: {:#?}", s);
    println!("Simple assignment: {:#?}", assignment_example);
    println!("If-else with complex guard: {:#?}", if_else_example);
    println!("Nested while loop: {:#?}", nested_while_example);
}
