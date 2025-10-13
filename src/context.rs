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
    pub tmp_count: u32,
    pub started_main: bool,
}

impl Context {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::default()],
            output: String::new(),
            errors: String::new(),
            sem_errors: 0,
            tmp_count: 0,
            started_main: false,
        }
    }

    pub fn append(&mut self, s: &str) {
        self.output.push_str("  ");
        self.output.push_str(s);
        self.output.push('\n');
    }

    pub fn start_main(&mut self) {
        self.append("define i32 @main() {");
        self.started_main = true;
    }

    pub fn end_main(&mut self) {
        if self.started_main {
            self.append("ret i32 0");
            self.append("}");
        }
    }

    /// Generate a new temporary register name to guarantee uniqueness
    pub fn new_tmp(&mut self) -> String {
        let register = format!("%t{}", self.tmp_count);
        self.tmp_count += 1;
        register
    }
}
