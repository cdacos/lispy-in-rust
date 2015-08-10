use std::fmt;

pub enum Value {
    Function(fn(Vec<Value>) -> Value),
    Number(f64),
    //TODO string: Option<String>,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Function(_) => write!(f, "Value: is a function{}", ""),
            Value::Number(n) => write!(f, "Value: {}", n),
        }
    }
}

impl Value {
    pub fn clone(&self) -> Value {
        match *self {
            Value::Function(f) => Value::Function(f),
            Value::Number(n) => Value::Number(n),
        }
    }
}
