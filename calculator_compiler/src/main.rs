use ast::*;
use calculator_compiler::*;

fn main() {
    // Declare variable names as Rust variables
    let x = "x";
    let y = "y";
    let z = "z";

    // Original statement
    let s = stmt! {
        (while (5) {
            (if (3) {
                (x = 5);
                (if (5) {
                    (y = ((x + 3) - 5))
                } else {
                })
            } else {
            })
        })
    };

    // New complex statement
    let new_statement = stmt! {
        (if ((x * 2) > 10) {
            (y = (x + 1));
            (z = (y * 2));
            (if ((z - y) < 5) {
                (x = (z / 2))
            } else {
                (x = 0)
            })
        } else {
            (while ((x + y) < 20) {
                (x = (x + 1));
                (y = (y + 1));
                (if ((x % 2) == 0) {
                    (z = (x * 3))
                } else {
                    (z = (y / 2))
                })
            })
        })
    };

    println!("Original: {:#?}", s);
    println!("New Statement: {:#?}", new_statement);
}
