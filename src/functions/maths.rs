use value::Value;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Minimum,
    Maximum,
}

fn operate(args: Vec<Value>, op: Operation) -> Value {
    let mut result;

    match args.len() {
        0 => panic!("Math operations require at least one number"),
        _ => {
            match args[0] {
                Value::Number(n) => result = n,
                _ => panic!("Math operations require numbers"),
            }
            for i in 1..args.len() {
                match args[i] {
                    Value::Number(n) =>
                        match op {
                            Operation::Add => result += n,
                            Operation::Subtract => result -= n,
                            Operation::Multiply => result *= n,
                            Operation::Divide => result /= n,
                            Operation::Minimum => if n < result { result = n },
                            Operation::Maximum => if n > result { result = n },
                        },
                    _ => panic!("Match operations require numbers"),
                }
            }
        }
    }
    
    Value::Number(result)
}

pub fn add(args: Vec<Value>) -> Value {
    operate(args, Operation::Add)
}

pub fn subtract(args: Vec<Value>) -> Value {
    operate(args, Operation::Subtract)
}

pub fn multiply(args: Vec<Value>) -> Value {
    operate(args, Operation::Multiply)
}

pub fn divide(args: Vec<Value>) -> Value {
    operate(args, Operation::Divide)
}

pub fn min(args: Vec<Value>) -> Value {
    operate(args, Operation::Minimum)
}

pub fn max(args: Vec<Value>) -> Value {
    operate(args, Operation::Maximum)
}

