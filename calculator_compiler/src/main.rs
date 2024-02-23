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

    let corrected_statement = stmt! {
        // Direct support for complex expressions in conditions might not be present
        (if (x > 10) { // Simplified condition for demonstration
            (y = (x + 1));
            (z = (y * 2));
            (if (z < 5) { // Simplified condition
                (x = (z / 2))
            } else {
                (x = 0)
            })
        } else {
            (while (x < 20) { // Simplified condition
                (x = (x + 1));
                (y = (y + 1));
                // Simplified usage that might not directly match your complex condition needs
                (if (x == 0) { // Placeholder for a more complex condition
                    (z = (x * 3))
                } else {
                    (z = (y / 2))
                })
            })
        })
    };

    println!("Corrected Statement: {:#?}", corrected_statement);
    println!("Original: {:#?}", s);
}
