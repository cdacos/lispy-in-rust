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
        io::stdout().flush().ok(); // Unexpectedly, it doesn't flush by itself

        let mut expression = String::new();

        io::stdin().read_line(&mut expression)
            .ok()
            .expect("failed to read line");

        match eval(parse(&*expression), env) {
            Value::Number(n) => println!("{}", f64::to_string(&n)),
            _ => println!("Expression did not return a number"),
        }
    }
}

// -----

fn main() {
    let mut env = standard_env();
    repl("lispy-in-rust> ", &mut env);
}
