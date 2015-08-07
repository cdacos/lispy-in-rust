use std::collections::HashMap;
use std::f32::consts::PI;
use parse::AstNode;
use value::Value;
use functions;

// An environment with some Scheme standard procedures.
pub fn standard_env() -> HashMap<String, Value> {
    let mut env: HashMap<String, Value> = HashMap::new();
    env.insert(string_from_str("pi"), Value::number(PI));

    env.insert(string_from_str("+"), Value::function(functions::maths::add));
    env.insert(string_from_str("-"), Value::function(functions::maths::subtract));
    env.insert(string_from_str("*"), Value::function(functions::maths::multiply));
    env.insert(string_from_str("/"), Value::function(functions::maths::divide));
    env.insert(string_from_str("max"), Value::function(functions::maths::max));
    env.insert(string_from_str("min"), Value::function(functions::maths::min));

    env.insert(string_from_str("="), Value::function(functions::operators::eq));
    env.insert(string_from_str("<"), Value::function(functions::operators::lt));
    env.insert(string_from_str("<="), Value::function(functions::operators::lte));
    env.insert(string_from_str(">"), Value::function(functions::operators::gt));
    env.insert(string_from_str(">="), Value::function(functions::operators::gte));
    env.insert(string_from_str("not"), Value::function(functions::operators::not));

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
        Value::number(atom.number.unwrap())
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
                    let test_result = eval(test, env).number.unwrap();
                    
                    return eval(if test_result != 0f32 { conseq } else { alt }, env);
                },
                "define" => { // (define var exp)
                    match list.remove(0).symbol {
                        Some(v) => {
                            let exp = eval(list.remove(0), env);
                            env.insert(v.clone(), exp);
                            return Value::new(None, None);
                        },
                        None => panic!("Define must be in the form: (define var exp)"),
                    }
                },
                _ => {}
            }
        }

        //  (proc arg...)
        let op = eval(first, env).function.unwrap();
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
