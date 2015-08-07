use value::Value;

pub fn add(args: Vec<Value>) -> Value {
    let mut result = 0f32;

    match args.len() {
        0 => panic!("Add needs at least one number"),
        _ => for arg in args {
                result += arg.number.unwrap();
        }
    }
    
    Value::number(result)
}

pub fn subtract(args: Vec<Value>) -> Value {
    let mut result = 0f32;

    match args.len() {
        0 => panic!("Subtract needs at least one number"),
        _ => for arg in args {
                result -= arg.number.unwrap();
        }
    }
    
    Value::number(result)
}

pub fn multiply(args: Vec<Value>) -> Value {
    let mut result = 1f32;

    match args.len() {
        0 => panic!("Multiply needs at least one number"),
        _ => for arg in args {
                result *= arg.number.unwrap();
        }
    }
    
    Value::number(result)
}

pub fn divide(args: Vec<Value>) -> Value {
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

pub fn min(args: Vec<Value>) -> Value {
    let mut result;

    match args.len() {
        0 => panic!("Min needs at least one number"),
        _ => {
            result = args[0].number.unwrap();
            for i in 1..args.len() {
                let a = args[i].number.unwrap();
                result = if a < result { a } else { result };
            }
        }
    }
    
    Value::number(result)
}

pub fn max(args: Vec<Value>) -> Value {
    let mut result;

    match args.len() {
        0 => panic!("Min needs at least one number"),
        _ => {
            result = args[0].number.unwrap();
            for i in 1..args.len() {
                let a = args[i].number.unwrap();
                result = if a > result { a } else { result };
            }
        }
    }
    
    Value::number(result)
}

