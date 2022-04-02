mod lisp;
mod object;
mod stdenv;
mod errors;

use std::env;
use std::fs;
use lisp::LispScope;

fn main() {
    let args: Vec<String> = env::args().collect();

    let code = if args.len() > 1 {
        fs::read_to_string(&args[1]).unwrap()
    } else {
        "(while t (print (eval (read))))".to_string()
    };

    let ret = LispScope::new(None)
        .add_stdenv()
        .eval(&code);

    match ret {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}
