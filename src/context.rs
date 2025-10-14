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

    pub fn error(&mut self, msg: &str) {
        self.errors.push_str("SEMANTIC ERROR: ");
        self.errors.push_str(msg);
        self.errors.push('\n');
        self.sem_errors += 1;
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

    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    pub fn leave_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn current_scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().expect("No scope available")
    }

    /// Look up a variable by name, searching from the innermost scope outward
    pub fn lookup(&self, name: &str) -> Option<&ValueObj> {
        for s in self.scopes.iter().rev() {
            if let Some(v) = s.vars.get(name) {
                return Some(v);
            }
        }
        None
    }

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut ValueObj> {
        for s in self.scopes.iter_mut().rev() {
            if let Some(v) = s.vars.get_mut(name) {
                return Some(v);
            }
        }
        None
    }
}
