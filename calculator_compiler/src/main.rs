use ast::*;
use calculator_compiler::*;

fn main() {
    // Declare variable names as Rust variables
    let x = "foo";
    let y = "bar";
    let z = "baz";

    let s = stmt! {
        (while (x < 10) {
            (if ((x + 2) <= 10) {
                (x = (x + 1)); // Increment x
            } else {
                // Ensure a no-op or valid statement here if your macro doesn't support empty blocks
            };)
            (z = ((x * 2) / (y + 3)));
        };)
    };

    println!("While with nested if-else: {:#?}", s);
}
