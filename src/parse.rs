#[derive(Debug)]
pub enum AstNode {
    List(Vec<AstNode>),
    Symbol(String),
    Number(f64),
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
pub fn parse(program: &str) -> AstNode {
    read_from_tokens(&mut tokenize(program))
}

// Read an expression from a sequence of tokens.
fn read_from_tokens(tokens: &mut Vec<String>) -> AstNode {
    let token = tokens.remove(0);
    match &*token {
        "(" => {
            let mut list = vec![];
            while tokens[0] != ")" {
                list.push(read_from_tokens(tokens));
            }
            tokens.remove(0); // pop off ')'
            AstNode::List(list)
        },
        ")" => panic!("unexpected token"),
        _ => atom(token)
    }
}

// Numbers become numbers; every other token is a symbol.
fn atom(token: String) -> AstNode {
    match token.parse::<f64>() {
        Ok(v) => AstNode::Number(v),
        _ => AstNode::Symbol(token),
    }
}
