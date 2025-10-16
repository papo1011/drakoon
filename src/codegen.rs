use crate::{
    ast::{Expr, Op, Stmt},
    types::{Type, Value, ValueObj, types_compatible},
};
use std::{collections::HashMap, vec};

#[derive(Default)]
pub struct Scope {
    pub vars: HashMap<String, ValueObj>,
}

pub struct CodeGen {
    pub scopes: Vec<Scope>,
    pub output: String,
    pub errors: String,
    pub sem_errors: u32,
    pub tmp_count: u32,
    pub started_main: bool,
}

impl CodeGen {
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
        if self.started_main {
            return;
        }
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

    pub fn is_name_available(&self, name: &str) -> bool {
        !self.scopes.last().unwrap().vars.contains_key(name)
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

    /// A left-hand value (lvalue) denote a memory address
    /// This function looks up a variable and returns its address Value
    ///
    /// If not found, it reports an error and returns a dummy Value
    pub fn lookup_lvalue(&mut self, name: &str) -> Value {
        if let Some(v) = self.lookup(name) {
            return v.val.clone();
        }
        self.error(&format!("Use of undeclared variable '{}'", name));
        Value::new_addr("%undef", Type::Unknown)
    }

    /// A right-hand value (rvalue) is a value used in expressions
    /// If the input Value is an address, it generates a load instruction to get the actual value
    /// If the input Value is already a value, it returns it as is.
    ///
    /// If the input Value is an address of an array, it reports an error since arrays
    /// cannot be used directly in expressions.
    pub fn rvalue(&mut self, v: &Value) -> Value {
        if v.is_addr {
            if matches!(v.ty, Type::Array(_, _)) {
                self.error(
                    "Cannot use array value directly in expression; index it or pass its address.",
                );
                return Value::new_val("%undef", Type::Unknown);
            }
            self.load_scalar(v)
        } else {
            v.clone()
        }
    }

    pub fn load_scalar(&mut self, addr: &Value) -> Value {
        let tmp = self.new_tmp();
        self.append(&format!(
            "{} = load {}, {}* {}, align {}",
            tmp,
            addr.ty.llvm(),
            addr.ty.llvm(),
            addr.repr,
            addr.ty.align()
        ));
        Value::new_val(tmp, addr.ty.clone())
    }

    pub fn store_scalar(&mut self, dst_addr: &Value, src_val: &Value) {
        let rhs = self.rvalue(src_val);
        if !types_compatible(&dst_addr.ty, &rhs.ty) {
            self.error(&format!(
                "Type mismatch in store: {} <- {}",
                dst_addr.ty, rhs.ty
            ));
            return;
        }
        self.append(&format!(
            "store {} {}, {}* {}, align {}",
            dst_addr.ty.llvm(),
            rhs.repr,
            dst_addr.ty.llvm(),
            dst_addr.repr,
            dst_addr.ty.align()
        ));
    }

    /// Allocates space on the stack for a variable of the given type and returns its address as a Value.
    pub fn alloca_of_type(&mut self, name: &str, ty: &Type) -> Value {
        let repr = format!("%{}", name);
        self.append(&format!(
            "{} = alloca {}, align {}",
            repr,
            ty.llvm(),
            ty.align()
        ));
        Value::new_addr(repr, ty.clone())
    }

    pub fn var_definition(
        &mut self,
        name: String,
        annot: Option<Type>,
        init: Value,
        mutable: bool,
    ) {
        if !self.is_name_available(&name) {
            self.error(&format!(
                "Variable '{}' already declared in this scope",
                name
            ));
            return;
        }

        let ty = if let Some(t) = annot {
            if !types_compatible(&t, &init.ty) {
                self.error(&format!(
                    "Type mismatch in declaration of '{}': {} <- {}",
                    name, t, init.ty
                ));
                return;
            }
            t
        } else {
            init.ty.clone()
        };

        let addr = self.alloca_of_type(&name, &ty);
        self.store_scalar(&addr, &init);

        let vo = ValueObj {
            name: name.clone(),
            val: addr,
            mutable,
        };
        self.current_scope_mut().vars.insert(name, vo);
    }

    pub fn var_assignment(&mut self, name: &str, value: Value) {
        let addr = match self.lookup(name) {
            Some(var) if var.mutable => var.val.clone(),
            Some(_) => {
                self.error(&format!("Cannot assign to immutable variable '{}'", name));
                return;
            }
            None => {
                self.error(&format!("Use of undeclared variable '{}'", name));
                return;
            }
        };

        let rhs = self.rvalue(&value);
        if !types_compatible(&addr.ty, &rhs.ty) {
            self.error(&format!(
                "Type mismatch in assignment to '{}': {} <- {}",
                name, addr.ty, rhs.ty
            ));
            return;
        }

        self.store_scalar(&addr, &rhs);
    }

    pub fn append_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDef {
                name,
                annot,
                value,
                mutable,
            } => {
                let init = self.append_expr(value);
                self.var_definition(name.clone(), annot.clone(), init, *mutable);
            }
            Stmt::VarAssign { name, value } => {
                let val = self.append_expr(value);
                self.var_assignment(name, val);
            }
            Stmt::Print { value } => {
                self.append_expr(value);
            }
        }
    }

    pub fn append_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Int(i) => Value::new_val(i.to_string(), Type::Int),
            Expr::Double(d) => Value::new_val(format!("{}", d), Type::Double),
            Expr::Var(name) => {
                let addr = self.lookup_lvalue(name);
                self.rvalue(&addr)
            }
            Expr::BinaryOp { lhs, operator, rhs } => {
                let lhs_val = self.append_expr(lhs);
                let rhs_val = self.append_expr(rhs);
                self.binop(operator.clone(), &lhs_val, &rhs_val)
            }
        }
    }

    pub fn binop(&mut self, op: Op, lhs: &Value, rhs: &Value) -> Value {
        let lhs = self.rvalue(lhs);
        let rhs = self.rvalue(rhs);

        if lhs.ty != rhs.ty {
            self.error("Type mismatch in binary expression.");
            return Value::new_val("%undef", Type::Unknown);
        }

        let instr = match (&lhs.ty, op) {
            (Type::Int, Op::Add) => "add",
            (Type::Int, Op::Sub) => "sub",
            (Type::Int, Op::Mul) => "mul",
            (Type::Int, Op::Div) => "sdiv",
            (Type::Double, Op::Add) => "fadd",
            (Type::Double, Op::Sub) => "fsub",
            (Type::Double, Op::Mul) => "fmul",
            (Type::Double, Op::Div) => "fdiv",
            _ => {
                self.error("Unsupported operand type for binary expression.");
                return Value::new_val("%undef", Type::Unknown);
            }
        };

        let tmp = self.new_tmp();
        self.append(&format!(
            "{} = {} {} {}, {}",
            tmp,
            instr,
            lhs.ty.llvm(),
            lhs.repr,
            rhs.repr
        ));
        Value::new_val(tmp, lhs.ty)
    }
}
