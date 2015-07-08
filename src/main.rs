use std::collections::HashMap;

#[derive(Debug)]
struct AstNode {
    symbol: Option<String>,
    list: Vec<Box<AstNode>>,
    number: Option<f32>,
}

impl AstNode {
    fn new() -> AstNode {
        AstNode {
            symbol: None,
            list: vec![],
            number: None,
        }
    }
}

fn tokenise(s: &str) -> Vec<String> {
    let spaced = s.replace("(", " ( ").replace(")", " ) ");
    spaced.split_whitespace()
        .filter(|&x| x.trim() != "")
        .map(|x| x.to_string())
        .collect()
}

fn atom(node: &mut AstNode, token: &str) {
    match token.parse::<f32>() {
        Ok(f) => node.number = Some(f),
        Err(_) => node.symbol = Some(token.to_string()),
    }
}

fn read_from_tokens(tokens: &mut Vec<String>) -> Box<AstNode> {
    let mut node = AstNode::new();
    let token = tokens.remove(0);
    match token.as_ref() {
        "(" => {            
            while tokens[0] != ")" {
                node.list.push(read_from_tokens(tokens));
            }
            tokens.remove(0); // remove ")"
        },
        ")" => panic!("unexpected token"),
        _ => {
            atom(&mut node, token.as_ref());
        }
    };
    Box::new(node)
}

#[allow(dead_code)]
fn print_ast(node: &AstNode, depth: i32) {
    let spaces = (1..depth).map(|_| " ").collect::<String>();
    let symbol = node.symbol.clone();

    match node.number {
        Some(f) => println!("{}{}", spaces, f),
        None => (),
    }

    match symbol {
        Some(s) => println!("{}{}", spaces, s),
        None => (),
    }

    if node.list.len() > 0 {
        println!("{}(", spaces);
        for child in &node.list {
            print_ast(&child, depth + 1);
        }
        println!("{})", spaces);
    }
}

struct EnvObj {
    f0: Option<fn() -> f32>,
    f1: Option<fn(x: f32) -> f32>,
    f2: Option<fn(x: f32, y: f32) -> f32>,
    number: Option<f32>,
}

impl EnvObj {
    fn new() -> EnvObj {
        EnvObj {
            f0: None,
            f1: None,
            f2: None,
            number: None,
        }
    }

    #[allow(dead_code)]
    fn f0(f: fn() -> f32) -> EnvObj {
        let mut env_obj = EnvObj::new();
        env_obj.f0 = Some(f);
        env_obj
    }

    #[allow(dead_code)]
    fn f1(f: fn(x: f32) -> f32) -> EnvObj {
        let mut env_obj = EnvObj::new();
        env_obj.f1 = Some(f);
        env_obj
    }

    fn f2(f: fn(x: f32, y: f32) -> f32) -> EnvObj {
        let mut env_obj = EnvObj::new();
        env_obj.f2 = Some(f);
        env_obj
    }

    fn n(n: f32) -> EnvObj {
        let mut env_obj = EnvObj::new();
        env_obj.number = Some(n);
        env_obj
    }

    fn clone(&self) -> EnvObj {
        let mut env_obj = EnvObj::new();
        env_obj.f0 = self.f0;
        env_obj.f1 = self.f1;
        env_obj.f2 = self.f2;
        env_obj.number = self.number;
        env_obj
    }
}

fn add(x: f32, y: f32) -> f32 {
    x + y
}

fn multiply(x: f32, y: f32) -> f32 {
    x * y
}

fn standard_env<'a>() -> HashMap<&'a str, EnvObj> {
    let mut env: HashMap<&'a str, EnvObj> = HashMap::new();
    env.insert("pi", EnvObj::n(3.1415926535f32));
    env.insert("+", EnvObj::f2(add));
    env.insert("*", EnvObj::f2(multiply));
    env
}

fn eval(x: AstNode, env: &HashMap<&str, EnvObj>) -> EnvObj {
    if x.symbol != None {
        let s: &str = &*x.symbol.unwrap();
        env.get(s).unwrap().clone()
    }
    else if x.number != None {
        EnvObj::n(x.number.unwrap())
    }
    else {
        let mut args: Vec<Box<AstNode>> = x.list;
        let node: AstNode = *args.remove(0);
        let f = eval(node, env);
        let mut results: Vec<EnvObj> = vec![];
        for arg in args {
            results.push(eval(*arg, &env));
        }
        match results.len() {
            0 => EnvObj::n(f.f0.unwrap()()),
            1 => EnvObj::n(f.f1.unwrap()(results[0].number.unwrap())),
            _ => EnvObj::n(f.f2.unwrap()(results[0].number.unwrap(), results[1].number.unwrap())),
        }
    }
}

fn main() {
    //let expression = "(begin (define r 10) (* pi (* r r)))";
    let expression = "(* 2 (+ 3 4))";
    let ast = read_from_tokens(&mut tokenise(expression));

    //print_ast(&ast, 1);
    //println!("{:#?}", &ast); // Nice pretty debug print

    let result = eval(*ast, &standard_env());
    println!("{} = {:#?}", expression, result.number.unwrap());
}
