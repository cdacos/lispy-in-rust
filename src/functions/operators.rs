use value::Value;

pub fn eq(args: Vec<Value>) -> Value {
    let mut result;

    match args.len() {
        0 => panic!("Equals needs at least one number"),
        _ => {
            result = args[0].number.unwrap();
            for i in 1..args.len() {
                let a = args[i].number.unwrap();
                result = if result == a { 1f32 } else { 0f32 };
            }
        }
    }
    
    Value::number(result)
}

pub fn lt(args: Vec<Value>) -> Value {
    let mut result;

    match args.len() {
        0 => panic!("Less-than needs at least one number"),
        _ => {
            result = args[0].number.unwrap();
            for i in 1..args.len() {
                let a = args[i].number.unwrap();
                result = if result < a { 1f32 } else { 0f32 };
            }
        }
    }
    
    Value::number(result)
}

pub fn lte(args: Vec<Value>) -> Value {
    let mut result;

    match args.len() {
        0 => panic!("Less-than-or-equal needs at least one number"),
        _ => {
            result = args[0].number.unwrap();
            for i in 1..args.len() {
                let a = args[i].number.unwrap();
                result = if result <= a { 1f32 } else { 0f32 };
            }
        }
    }
    
    Value::number(result)
}

pub fn gt(args: Vec<Value>) -> Value {
    let mut result;

    match args.len() {
        0 => panic!("Greater-than needs at least one number"),
        _ => {
            result = args[0].number.unwrap();
            for i in 1..args.len() {
                let a = args[i].number.unwrap();
                result = if result > a { 1f32 } else { 0f32 };
            }
        }
    }
    
    Value::number(result)
}

pub fn gte(args: Vec<Value>) -> Value {
    let mut result;

    match args.len() {
        0 => panic!("Greater-than-or-equal needs at least one number"),
        _ => {
            result = args[0].number.unwrap();
            for i in 1..args.len() {
                let a = args[i].number.unwrap();
                result = if result >= a { 1f32 } else { 0f32 };
            }
        }
    }
    
    Value::number(result)
}

pub fn not(args: Vec<Value>) -> Value {
    let mut result = 0f32;

    match args.len() {
        0 => panic!("Not needs at least one number"),
        _ => for arg in args {
                result += arg.number.unwrap();
        }
    }
    
    Value::number(if result != 0f32 { 0f32 } else { 1f32 })
}

