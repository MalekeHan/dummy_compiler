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
    let a = "a";
    let b = "b";
    let i = "i";
    let temp = "temp";

    // let block = block! ({
    //     // Initial assignments
    //     (n = 5);
    //     (a = 0); // fib0 
    //     (b = 1); // fib1 
    //     (i = 1);
    
    //     // Loop to calculate Fibonacci iteratively
    //     (while ((i < n)) {
    //         // Temporary variable to hold fib1 for the update
    //         (temp = b);
    //         (b = (b + a)); // New fib1 is the sum of fib0 and fib1
    //         (a = temp); // Update fib0 to the old fib1
    //         (i = (i + 1))
    //     })
    // });

    let block = block! ({
        (n = (3 < 5))
    });
    

    compiler.compile(&block); //&s or &[s]

    // Save the compiled LLVM IR to a file
    compiler
        .module
        .print_to_file(Path::new("output.ll"))
        .expect("failed");

    // Optionally, print the LLVM IR to stdout
    let llvm_ir = compiler.module.print_to_string().to_string();
    println!("LLVM IR:\n{}", llvm_ir);
}
