#[derive(Debug)]
pub struct AstNode {
    pub list: Vec<AstNode>,
    pub symbol: Option<String>,
    pub number: Option<f64>,
}

impl AstNode {
    pub fn new() -> AstNode {
        AstNode {
            list: vec![],
            symbol: None,
            number: None,
        }
    } 

    pub fn clone(&self) -> AstNode {
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
pub fn parse(program: &str) -> AstNode {
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
    match token.parse::<f64>() {
        Ok(v) => atom.number = Some(v),
        _ => atom.symbol = Some(token),
    }
    atom
}
