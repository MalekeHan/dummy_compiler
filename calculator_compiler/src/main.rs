mod compiler;
use calculator_compiler::Compiler;
use ast::*;
use std::path::Path;

fn main() {
    let context = Context::create();
    let mut compiler = Compiler::new(&context);

    // you need to declare variable names as rust variables
    let x = "foo";
    let y = "bar";

    let s = stmt! {
        // parentheses around statements, and parentheses in guards are mandatory
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

    compiler.compile(&[s]);

    // Save the compiled LLVM IR to a file
    compiler.module.print_to_file(Path::new("output.ll")).expect("failed");

    // Optionally, print the LLVM IR to stdout
    let llvm_ir = compiler.module.print_to_string().to_str().unwrap();
    println!("LLVM IR:\n{}", llvm_ir);
}
