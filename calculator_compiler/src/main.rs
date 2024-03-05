use ast::*;
use calculator_compiler::*;
use compiler::*;
use inkwell::context::Context;
use std::path::Path;

fn main() {
    let context = Context::create();
    let mut compiler = Compiler::new(&context);

    // // you need to declare variable names as rust variables
    // let x = "foo";
    // let y = "bar";

    // let s = stmt! {
    //     // parentheses around statements, and parentheses in guards are mandatory
    //     (while (5) {
    //         (if (3) {
    //             (x = 5);
    //             (if (5) {
    //                 (y = ((x + 3) - 5))
    //             } else {
    //             })
    //         } else {
    //         })
    //     })
        
    // };

    let n = "n"; // Assuming n is the input number for which we want to calculate the Fibonacci number
    let fib = "fib";
    let a = "a";
    let b = "b";
    let i = "i";
    let temp = "foo";

    let s = block! {
        // Initial assignments
        (n = 5);
        (a = 0); // Initialize a to 0
        (b = 1); // Initialize b to 1
        (i = 1); // Initialize i to 1
        
        // Loop to calculate Fibonacci iteratively
        ((while ((i < n)) {
            {
                (temp = b); // Temporary variable to hold b for the update
                (b = ((b + a))); // New b is the sum of a and b
                (a = temp); // Update a to the old b
                (i = ((i + 1))); // Increment i
            }
        }))
    };
    
    // Note: This example assumes that you have a mechanism outside of these macros to execute or evaluate the `block!`.
    
    

    compiler.compile(&[s]);

    // Save the compiled LLVM IR to a file
    compiler
        .module
        .print_to_file(Path::new("output.ll"))
        .expect("failed");

    // Optionally, print the LLVM IR to stdout
    let llvm_ir = compiler.module.print_to_string().to_string();
    println!("LLVM IR:\n{}", llvm_ir);
}
