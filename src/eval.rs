use std::f64::consts::PI;
use env::Env;
use parse::AstNode;
use value::Value;
use functions;

// An environment with some Scheme standard procedures.
pub fn standard_env() -> Env {
    let mut env: Env = Env::new(None);
    env.set("pi", Value::Number(PI));

    env.set("+", Value::Function(functions::maths::add));
    env.set("-", Value::Function(functions::maths::subtract));
    env.set("*", Value::Function(functions::maths::multiply));
    env.set("/", Value::Function(functions::maths::divide));
    env.set("max", Value::Function(functions::maths::max));
    env.set("min", Value::Function(functions::maths::min));

    env.set("=", Value::Function(functions::operators::eq));
    env.set("<", Value::Function(functions::operators::lt));
    env.set("<=", Value::Function(functions::operators::lte));
    env.set(">", Value::Function(functions::operators::gt));
    env.set(">=", Value::Function(functions::operators::gte));
    env.set("not", Value::Function(functions::operators::not));

    env
}

// Evaluate an expression in an environment.
pub fn eval(atom: AstNode, env: &mut Env) -> Value {
    match atom {
        AstNode::Symbol(symbol) => { // variable reference
            env.find(&*symbol).clone()
        },
        AstNode::Number(n) => { // constant literal
            Value::Number(n)
        },
        AstNode::List(l) => { // list
            let mut list = l;
            let first: AstNode = list.remove(0);
            let s = match first {
                AstNode::Symbol(ref s) => s.clone(),
                _ => String::new(),
            };

            match &*s {
                "quote" => { // (quote exp)
                    if list.len() != 1 {
                        panic!("Quote must be in the form of: (quote exp)");
                    }
                    let exp = list.remove(0);
                    return eval(exp, env);
                },
                "if" => { // (if test conseq alt)
                    if list.len() != 3 {
                        panic!("If must be in the form of: (if test conseq alt)");
                    }
                    let test = list.remove(0);
                    let conseq = list.remove(0);
                    let alt = list.remove(0);
                    let test_result = match eval(test, env) {
                        Value::Number(n) => n,
                        _ => panic!("Eval value is expected to be a number."),
                    };
                    return eval(if test_result != 0f64 { conseq } else { alt }, env);
                },
                "define" => { // (define var exp)
                    match list.remove(0) {
                        AstNode::Symbol(v) => {
                            let exp = eval(list.remove(0), env);
                            env.set(v, exp);
                            return Value::Number(1f64);
                        },
                        _ => panic!("Define must be in the form: (define var exp)"),
                    }
                },
                "set!" => { // (set! var exp)
                    match list.remove(0) {
                        AstNode::Symbol(v) => {
                            let exp = eval(list.remove(0), env);
                            env.find_and_set(v, exp);
                            return Value::Number(1f64);
                        },
                        _ => panic!("Define must be in the form: (set! var exp)"),
                    }
                }
                _ => { //  (proc arg...)
                    let op = match eval(first, env) {
                        Value::Function(f) => f,
                        _ => panic!("Procedure expected"),
                    };
                    let mut args: Vec<Value> = vec![];
                    for arg in list {
                        args.push(eval(arg, env));
                    }
                    op(args)
                },
            }
        },
    }
}
