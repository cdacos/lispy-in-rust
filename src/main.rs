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
    spaced.split(" ")
        .filter(|&x| x.trim() != "")
        .map(|x| x.to_string())
        .collect()
}

fn atom<'a>(node: &'a mut AstNode, token: &'a str) {
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

fn print_ast(node: &AstNode, depth: i32) {
    let spaces = (1..depth).map(|_| " ").collect::<String>();
    let symbol = node.symbol.clone();

    match node.number {
        Some(f) => println!("{}{},", spaces, f),
        None => (),
    }

    match symbol {
        Some(s) => println!("{}{},", spaces, s),
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

fn main() {
    let expression = "(begin (define r 10) (* pi (* r r)))";

    let ast = read_from_tokens(&mut tokenise(expression));

    print_ast(&ast, 1);
}
