use std::fmt;

pub struct Value {
    pub function: Option<fn(x: Vec<Value>) -> Value>,
    pub number: Option<f32>,
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
    pub fn new(f: Option<fn(x: Vec<Value>) -> Value>, n: Option<f32>) -> Value {
        Value {
            function: f,
            number: n,
        }
    }
    
    pub fn function(f: fn(x: Vec<Value>) -> Value) -> Value {
        Value::new(Some(f), None)
    }
    
    pub fn number(n: f32) -> Value {
        Value::new(None, Some(n))
    }

    pub fn clone(&self) -> Value {
        Value::new(self.function, self.number)
    }
}
