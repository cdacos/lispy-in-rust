use std::collections::HashMap;
use std::f64::consts::PI;
use parse::AstNode;
use value::Value;
use functions;

// An environment with some Scheme standard procedures.
pub fn standard_env() -> HashMap<String, Value> {
    let mut env: HashMap<String, Value> = HashMap::new();
    env.insert(string_from_str("pi"), Value::Number(PI));

    env.insert(string_from_str("+"), Value::Function(functions::maths::add));
    env.insert(string_from_str("-"), Value::Function(functions::maths::subtract));
    env.insert(string_from_str("*"), Value::Function(functions::maths::multiply));
    env.insert(string_from_str("/"), Value::Function(functions::maths::divide));
    env.insert(string_from_str("max"), Value::Function(functions::maths::max));
    env.insert(string_from_str("min"), Value::Function(functions::maths::min));

    env.insert(string_from_str("="), Value::Function(functions::operators::eq));
    env.insert(string_from_str("<"), Value::Function(functions::operators::lt));
    env.insert(string_from_str("<="), Value::Function(functions::operators::lte));
    env.insert(string_from_str(">"), Value::Function(functions::operators::gt));
    env.insert(string_from_str(">="), Value::Function(functions::operators::gte));
    env.insert(string_from_str("not"), Value::Function(functions::operators::not));

    env
}

fn string_from_str(string: &str) -> String {
    let mut s = String::new();
    s.push_str(string);
    s
}

// Evaluate an expression in an environment.
pub fn eval(atom: AstNode, env: &mut HashMap<String, Value>) -> Value {
    if atom.symbol != None { // variable reference
        let s: &str = &*atom.symbol.unwrap();
        env.get(s).unwrap().clone()
    }
    else if atom.number != None { // constant literal
        Value::Number(atom.number.unwrap())
    }
    else if atom.list.len() > 0 {  // non-empty list
        let mut list = atom.list;
        let first: AstNode = list.remove(0);
        let first_symbol = first.clone().symbol;
        
        if first_symbol != None {
            match &*first_symbol.unwrap() {
                "quote" =>  { // (quote exp)
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
                    match list.remove(0).symbol {
                        Some(v) => {
                            let exp = eval(list.remove(0), env);
                            env.insert(v.clone(), exp);
                            return Value::Number(1f64);
                        },
                        None => panic!("Define must be in the form: (define var exp)"),
                    }
                },
                _ => {}
            }
        }

        //  (proc arg...)
        let op = match eval(first, env) {
            Value::Function(f) => f,
            _ => panic!("Procedure expected"),
        };
        let mut args: Vec<Value> = vec![];
        for arg in list {
            args.push(eval(arg, env));
        }
        op(args)
    }
    else { // empty list
        panic!("TODO: Handle empty lists");
    }
}
