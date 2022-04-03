mod lisp;
mod object;
mod stdenv;
mod errors;

use std::collections::HashMap;

use std::env;
use std::fs;
use lisp::Lisp;

fn main() {
    let args: Vec<String> = env::args().collect();

    let code = if args.len() > 1 {
        fs::read_to_string(&args[1]).unwrap()
    } else {
        "(while t (print (eval (read))))".to_string()
    };

    let mut globals = HashMap::new();
    let ret = Lisp::new(&mut globals)
        .add_stdenv()
        .eval(&code);

    match ret {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}
