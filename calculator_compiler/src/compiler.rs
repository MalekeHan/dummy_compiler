use inkwell::{
    builder::{Builder, BuilderError},
    context::Context,
    module::Module,
    values::{FunctionValue, IntValue},
};
use std::collections::HashMap;

use crate::ast::*;

//struct to hold the state and tools to use LLVM
pub struct Compiler<'ctx> {
    context: &'ctx Context,
    pub module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<String, IntValue<'ctx>>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    // constructor for the compiler struct
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("my_compiler");
        let builder = context.create_builder();
        Compiler {
            context,
            module,
            builder,
            variables: HashMap::new(),
            fn_value_opt: None,
        }
    }

    // function to compile an expression AST node into LLVM IR return an LLVM int value
    fn compile_expr(&mut self, expr: &Expr) -> Result<IntValue<'ctx>, BuilderError> {
        match expr {
            Expr::Number(n) => Ok(self.context.i64_type().const_int(*n as u64, false)), // the expression is a number convert directly to LLVM
            Expr::Variable(name) => Ok(*self.variables.get(name).unwrap()), // retrueve the LLVM value from the variables map -- panic if not found in map

            // we want to recursively compile both the left and right operands first
            // after we can use the correct LLVM instruction for the operation
            Expr::BinaryOp(binary_op) => {
                let left = self.compile_expr(&binary_op.left)?;
                let right = self.compile_expr(&binary_op.right)?;
                match binary_op.kind {

                    // arithmetic ops

                    BinaryOpKind::Add => self.builder.build_int_add(left, right, "addtmp"),
                    BinaryOpKind::Subtract => self.builder.build_int_sub(left, right, "subtmp"),
                    BinaryOpKind::Multiply => self.builder.build_int_mul(left, right, "multmp"),
                    BinaryOpKind::Divide => self.builder.build_int_signed_div(left, right, "divtmp"),
                    
                    // comparison and equality ops
                    BinaryOpKind::GreaterThan => (self.builder.build_int_compare(IntPredicate::SGT, left, right, "gttmp")),
                    BinaryOpKind::LessThan => (self.builder.build_int_compare(IntPredicate::SLT, left, right, "lttmp")),
                    BinaryOpKind::GreaterThanOrEqual => (self.builder.build_int_compare(IntPredicate::SGE, left, right, "getmp")),
                    BinaryOpKind::LessThanOrEqual => (self.builder.build_int_compare(IntPredicate::SLE, left, right, "letmp")),
                    BinaryOpKind::Equal => (self.builder.build_int_compare(IntPredicate::EQ, left, right, "eqtmp")),
                    _ => unimplemented!(),
                }
            }
        }
    }

    fn compile_block(&mut self, block: &[Stmt]) {
        for stmt in block {
            self.compile_stmt(stmt);
        }
    }

    // function to compile an statement AST node to LLVM IR --> used for multiple kinds statements
    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), BuilderError> {
        match stmt {
            // handle the Assignment statemetns by compiling the expression on the right hand side and then store the var into the var map
            Stmt::Assignment(assignment) => {
                let value = self.compile_expr(&assignment.value)?;
                self.variables.insert(assignment.name.clone(), value); // insert the computed value into the var map [varName |--> value ]
            }

            Stmt::If(if_stmt) => {
                // we need to compile the condition into an LLVM val
                let condition = self.compile_expr(&if_stmt.guard)?;
                let parent = self.fn_value_opt.unwrap(); // get the current function being compiled

                // create all the basic blocks for if-then-else
                let then_bb = self.context.append_basic_block(parent, "then");
                let else_bb = self.context.append_basic_block(parent, "else");

                // we need to create the continuation block
                let cont_bb = self.context.append_basic_block(parent, "ifcont");

                // create a branhc instruction and jump based on the result of the guard
                self.builder
                    .build_conditional_branch(condition, then_bb, else_bb);

                // compiles statements in the then block
                self.builder.position_at_end(then_bb);
                for stmt in &if_stmt.true_branch {
                    self.compile_stmt(stmt)?;
                }

                // jump to the continuation block after the then block
                self.builder.build_unconditional_branch(cont_bb);

                // compiles statements in the else block
                self.builder.position_at_end(else_bb);
                for stmt in &if_stmt.false_branch {
                    self.compile_stmt(stmt)?;
                }

                // jump to the continuation block after the else  block
                self.builder.build_unconditional_branch(cont_bb);

                self.builder.position_at_end(cont_bb);
            }

            Stmt::While(while_stmt) => {
                let parent = self.fn_value_opt.unwrap(); // get the current function being compiled
                                                         // create all the bbs
                let loop_bb = self.context.append_basic_block(parent, "loop");
                let body_bb = self.context.append_basic_block(parent, "body");
                let cont_bb = self.context.append_basic_block(parent, "whilecont");

                // jumps to the loop block from the current position -- unconditionally since we entered
                self.builder.build_unconditional_branch(loop_bb);

                // compile the loop condition/guard and check if we need to enter the loop again
                self.builder.position_at_end(loop_bb);
                let condition = self.compile_expr(&while_stmt.guard)?;
                self.builder
                    .build_conditional_branch(condition, body_bb, cont_bb);

                // actually compile the loop body
                self.builder.position_at_end(body_bb);
                for stmt in &while_stmt.body {
                    self.compile_stmt(stmt);
                }

                // jumps back to check the condition again after completing the body
                self.builder.build_unconditional_branch(loop_bb);

                // jump to the continuation block after the loop
                self.builder.position_at_end(cont_bb);
            }
        }

        Ok(())
    }

    // main compilation function that will take the program in  (series of statements) and return LLVM IR
    // pub fn compile(&mut self, ast: &[Stmt]) {
    //     let i64_type = self.context.i64_type();
    //     let fn_type = i64_type.fn_type(&[], false);
    //     let function = self.module.add_function("main", fn_type, None);
    //     let basic_block = self.context.append_basic_block(function, "entry");

    //     self.builder.position_at_end(basic_block);
    //     self.fn_value_opt = Some(function);

    //     for stmt in ast {
    //         self.compile_stmt(stmt);
    //     }

    //     self.builder
    //         .build_return(Some(&i64_type.const_int(0, false)));
    // }

    pub fn compile(&mut self, ast: &Vec<Stmt>) {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);
        self.fn_value_opt = Some(function);

        // Since we're now working with a Vec<Stmt>, we can iterate over it directly.
        // There's no need to change how each statement is processed.
        for stmt in ast {
            // `compile_stmt` now consumes the statement, so ensure `compile_stmt` can accept owned Stmt if necessary.
            self.compile_stmt(&stmt).expect("Failed to compile statement");
        }

        self.builder
            .build_return(Some(&i64_type.const_int(0, false)));
    }
}

// fn main() {
//     let context = Context::create();
//     let mut compiler = Compiler::new(&context);

//     // Assume `ast` is your Abstract Syntax Tree parsed from source code
//     let ast = vec![
//         // Your AST nodes go here
//         // Example: Stmt::Assignment(Assignment { name: "x".to_string(), value: Expr::Number(42) }),
//     ];
//     compiler.compile(&ast);

//     compiler.module.print_to_string().to_str().unwrap();
//     compiler.module.print_to_file(Path::new("output.ll")).unwrap();
// }
