mod add;
mod div;
mod eval_expr;
mod mul;
mod sub;
mod tests;
use std::env;

use crate::eval_expr::eval;
use crate::tests::run_tests;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let expr = &args[1];
        let base = &args[2];
        let result = eval(expr, base, "+-*/%");

        match result {
            Ok(res_str) => {
                println!("{} = {}", expr, res_str);
            }
            Err(_e) => {
                println!("Error");
            }
        }
    } else {
        run_tests();
    }
}
