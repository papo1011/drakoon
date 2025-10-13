use std::{collections::HashMap, vec};

use crate::types::ValueObj;

#[derive(Default)]
pub struct Scope {
    pub vars: HashMap<String, ValueObj>,
}

pub struct Context {
    pub scopes: Vec<Scope>,
    pub output: String,
    pub errors: String,
    pub sem_errors: u32,
    pub temp_count: u32,
    pub started_main: bool,
}

impl Context {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::default()],
            output: String::new(),
            errors: String::new(),
            sem_errors: 0,
            temp_count: 0,
            started_main: false,
        }
    }
}
