use crate::token::Literal;
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment{
    pub fn new() -> Self{
        Environment{
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self,name: String,value: Literal){
        self.values.insert(name,value);
    }

    pub fn get(&self,name: String) -> Literal{
        // self.values.get(&name).unwrap().clone()
        self.values.get(&name).expect("Undefined variable").clone()
    }

    pub fn assign(&mut self,name: String,value: Literal){
        self.values.insert(name,value);
    }
}