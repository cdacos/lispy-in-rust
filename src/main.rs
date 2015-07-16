// See http://norvig.com/lispy.html

#[derive(Debug)]
struct AstNode {
    list: Vec<AstNode>,
    symbol: Option<String>,
    number: Option<f32>,
}

impl AstNode {
    fn new() -> AstNode {
        AstNode {
            list: vec![],
            symbol: None,
            number: None,
        }
    } 

    fn clone(&self) -> AstNode {
        let mut node = AstNode::new();
        node.number = self.number;
        match self.symbol {
            Some(ref s)  => node.symbol = Some(s.clone()),
            None => node.symbol = None,
        }
        node
    }
}

//Convert a string of characters into a list of tokens.
fn tokenize(s: &str) -> Vec<String> {
    s.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

// Read a Scheme expression from a string.
fn parse(program: &str) -> AstNode {
    read_from_tokens(&mut tokenize(program))
}

// Read an expression from a sequence of tokens.
fn read_from_tokens(tokens: &mut Vec<String>) -> AstNode {
    let token = tokens.remove(0);
    match &*token {
        "(" => {
            let mut atom = AstNode::new();
            while tokens[0] != ")" {
                atom.list.push(read_from_tokens(tokens));
            }
            tokens.remove(0); // pop off ')'
            atom
        },
        ")" => panic!("unexpected token"),
        _ => atom(token)
    }
}

// Numbers become numbers; every other token is a symbol.
fn atom(token: String) -> AstNode {
    let mut atom = AstNode::new();
    match token.parse::<f32>() {
        Ok(v) => atom.number = Some(v),
        _ => atom.symbol = Some(token),
    }
    atom
}

// -----
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fmt;

struct Value {
    function: Option<fn(x: Vec<Value>) -> Value>,
    number: Option<f32>,
    //TODO string: Option<String>,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.number != None {
            write!(f, "Value: {}", self.number.unwrap())
        }
        else {
            write!(f, "Value: Function")
        }
    }
}

impl Value {
    fn new(f: Option<fn(x: Vec<Value>) -> Value>, n: Option<f32>) -> Value {
        Value {
            function: f,
            number: n,
        }
    }
    
    fn function(f: fn(x: Vec<Value>) -> Value) -> Value {
        Value::new(Some(f), None)
    }
    
    fn number(n: f32) -> Value {
        Value::new(None, Some(n))
    }

    fn clone(&self) -> Value {
        Value::new(self.function, self.number)
    }
}

// An environment with some Scheme standard procedures.
fn standard_env() -> HashMap<String, Value> {
    let mut env: HashMap<String, Value> = HashMap::new();
    env.insert(string_from_str("pi"), Value::number(PI));
    env.insert(string_from_str("+"), Value::function(add));
    env.insert(string_from_str("-"), Value::function(subtract));
    env.insert(string_from_str("*"), Value::function(multiply));
    env.insert(string_from_str("/"), Value::function(divide));
    env
}

fn string_from_str(string: &str) -> String {
    let mut s = String::new();
    s.push_str(string);
    s
}

// -----


fn add(args: Vec<Value>) -> Value {
    let mut result = 0f32;

    match args.len() {
        0 => panic!("Add needs at least one number"),
        _ => for arg in args {
                result += arg.number.unwrap();
        }
    }
    
    Value::number(result)
}

fn subtract(args: Vec<Value>) -> Value {
    let mut result = 0f32;

    match args.len() {
        0 => panic!("Subtract needs at least one number"),
        _ => for arg in args {
                result -= arg.number.unwrap();
        }
    }
    
    Value::number(result)
}

fn multiply(args: Vec<Value>) -> Value {
    let mut result = 1f32;

    match args.len() {
        0 => panic!("Multiply needs at least one number"),
        _ => for arg in args {
                result *= arg.number.unwrap();
        }
    }
    
    Value::number(result)
}

fn divide(args: Vec<Value>) -> Value {
    let mut result;

    match args.len() {
        0 => panic!("Divide needs at least one number"),
        _ => {
            result = args[0].number.unwrap();
            for i in 1..args.len() {
                result /= args[i].number.unwrap();
            }
        }
    }
    
    Value::number(result)
}

// -----

// Evaluate an expression in an environment.
fn eval(atom: AstNode, env: &mut HashMap<String, Value>) -> Value {
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
                    panic!("TODO: Handle quote");
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

use std::io;

// A prompt-read-eval-print loop.
fn repl(prompt: &str, env: &mut HashMap<String, Value>) {
    loop {
        println!("{}", prompt);

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
    repl("rust-lispy> ", &mut env);
}
