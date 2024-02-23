use ast::*;
use calculator_compiler::*;

fn main() {
    // // you need to declare variable names as rust variables
    // let x = "foo";
    // let y = "bar";

    // let s = stmt! {
    //     // parentheses around statements, and parentheses in guards are mandatory
    //     (while (5) {
    //     (if (3) {
    //         (x = 5);
    //         (if (5) {
    //             (y = ((x + 3) - 5))
    //         } else {
    //         })
    //     } else {
    //     })
    //     })
    // };

    // println!("Built: {:#?}", s);
    
    let x = "x";
    let y = "y";
    let z = "z";

    let complex_logic = stmt! {
        (while (x < 10) {
            (if (x > 5) {
                (y = (y + 1));
            } else {
                (z = (z - 1));
            });
            (x = (x + 1));
        })
    };

    println!("Complex Logic: {:#?}", complex_logic);





}
