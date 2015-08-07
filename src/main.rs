// See http://norvig.com/lispy.html

mod value;
mod functions;
mod parse;
mod eval;

use std::io;
use std::io::Write;
use std::collections::HashMap;
use value::Value;
use parse::parse;
use eval::{eval, standard_env};

// A prompt-read-eval-print loop.
fn repl(prompt: &str, env: &mut HashMap<String, Value>) {
    println!("\n *** Lispy in Rust ***\n\nEnter your s-expression. Only supports arithmetic operations, for example: (+ 2 (* 3 pi)).\nEnter q or quit to exit.");

    loop {
        print!("{}", prompt);
        io::stdout().flush().ok();

        let mut expression = String::new();

        io::stdin().read_line(&mut expression)
            .ok()
            .expect("failed to read line");

        let ast = parse(&*expression);
        let t = eval(ast, env);

        if t.number != None {
            println!("{:#?}", t.number.unwrap());
        }
//        else {
//            println!("\n>>> No result returned");
//            println!("\n>>> The environment is: {:#?}", env);
//        }
    }
}

// -----

fn main() {
    let mut env = standard_env();
    repl("lispy-in-rust> ", &mut env);
}
