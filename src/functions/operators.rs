use value::Value;

enum Operation {
    Equals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
    Not,
}

fn operate(args: Vec<Value>, op: Operation) -> Value {
    let mut result = 1f64;

    match args.len() {
        0 => panic!("Math operations require at least one number"),
        _ => {
            let mut previous;
            match args[0] {
                Value::Number(n) => previous = n,
                _ => panic!("Math operations require numbers"),
            }
            for i in 1..args.len() {
                match args[i] {
                    Value::Number(n) =>
                        match op {
                            Operation::Equals => if previous == n { result = 0f64 },
                            Operation::LessThan => if previous >= n { result = 0f64 },
                            Operation::LessThanOrEquals => if previous > n { result = 0f64 },
                            Operation::GreaterThan => if previous <= n { result = 0f64 },
                            Operation::GreaterThanOrEquals => if previous < n { result = 0f64 },
                            Operation::Not => if n != 0f64 { result = 0f64 },
                        },
                    _ => panic!("Match operations require numbers"),
                }
                if result == 0f64 {
                    return Value::Number(result);
                }
            }
        }
    }
    
    Value::Number(result)
}



pub fn eq(args: Vec<Value>) -> Value {
    operate(args, Operation::Equals)
}

pub fn lt(args: Vec<Value>) -> Value {
    operate(args, Operation::LessThan)
}

pub fn lte(args: Vec<Value>) -> Value {
    operate(args, Operation::LessThanOrEquals)
}

pub fn gt(args: Vec<Value>) -> Value {
    operate(args, Operation::GreaterThan)
}

pub fn gte(args: Vec<Value>) -> Value {
    operate(args, Operation::GreaterThanOrEquals)
}

pub fn not(args: Vec<Value>) -> Value {
    operate(args, Operation::Not)
}

