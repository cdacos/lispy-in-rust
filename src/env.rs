use std::collections::HashMap;
use value::Value;

pub struct Env {
    dict: HashMap<String, Value>,
    outer: Option<Box<Env>>,
}

impl Env {
    pub fn new(outer: Option<Env>) -> Env {
        Env {
            dict: HashMap::new(),
            outer: match outer {
                Some(e) => Some(Box::new(e)),
                None => None,
            }
        }
    }

    pub fn set<S>(&mut self, var: S, value: Value) where S: Into<String> {
        let k = var.into();
        self.dict.remove(&k);
        self.dict.insert(k, value);
    }

    pub fn find<S>(&self, var: S) -> Value where S: Into<String> {
        let k = var.into();
        match self.dict.get(&k) {
            Some(v) => v.clone(),
            None => {
                match self.outer {
                    Some(ref e) => e.find(k),
                    None => Value::None
                }
            }
        }
    }

    pub fn find_and_set<S>(&mut self, var: S, value: Value) where S: Into<String> {
        let k = var.into();
        match self.dict.get_mut(&k) {
            Some(_) => self.set(k, value),
            None => {
                match self.outer {
                    Some(ref mut e) => e.find_and_set(k, value),
                    None => panic!(format!("Could not find variable {}", k)),
                }
            }
        }
    }
}

