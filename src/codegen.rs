use crate::{
    ast::{Expr, Op, Stmt},
    types::{Type, Value, ValueObj, types_compatible},
};
use std::collections::HashMap;

#[derive(Default)]
pub struct Scope {
    pub vars: HashMap<String, ValueObj>,
}

#[derive(Default, Clone)]
pub struct FnInfo {
    pub params: Vec<(String, Type)>,
    pub ret_type: Type,
}

pub struct CodeGen {
    pub output: String,
    pub errors: String,
    pub sem_errors: u32,
    pub functions: HashMap<String, FnInfo>,
    scopes: Vec<Scope>,
    tmp_count: u32,
    printf_declared: bool,
    string_literals: HashMap<String, (String, usize)>,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            errors: String::new(),
            sem_errors: 0,
            functions: HashMap::new(),
            scopes: vec![Scope::default()],
            tmp_count: 0,
            printf_declared: false,
            string_literals: HashMap::new(),
        }
    }

    pub fn append(&mut self, s: &str) {
        self.output.push_str("  ");
        self.output.push_str(s);
        self.output.push('\n');
    }

    pub fn newline(&mut self) {
        self.output.push('\n');
    }

    pub fn error(&mut self, msg: &str) {
        self.errors.push_str("SEMANTIC ERROR: ");
        self.errors.push_str(msg);
        self.errors.push('\n');
        self.sem_errors += 1;
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
            Stmt::MainDef { body } => {
                self.append_main(body);
            }
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
            Stmt::PrintExpr { value } => {
                self.append_print_expr(value);
            }
            Stmt::PrintString { value } => {
                self.append_print_string(value);
            }
            Stmt::FnDef {
                name,
                params,
                ret_type,
                body,
            } => {
                self.append_fn_def(name, params, ret_type, body);
            }
            Stmt::FnCall { name, args } => {
                self.append_fn_call(name, args);
            }
        }
    }

    /* -------------------------------------------------------------------------- */
    /*                                MAIN                                        */
    /* -------------------------------------------------------------------------- */

    pub fn start_main(&mut self) {
        if self.functions.contains_key("main") {
            self.error("Multiple 'main' function definitions.");
            return;
        }
        self.functions.insert("main".to_string(), FnInfo::default());
        self.append("define i32 @main() {");
    }

    pub fn end_main(&mut self) {
        if self.functions.contains_key("main") {
            self.append("ret i32 0");
            self.append("}");
        }
    }

    pub fn append_main(&mut self, body: &[Stmt]) {
        self.start_main();
        for stmt in body {
            self.append_stmt(stmt);
        }
        self.end_main();
    }

    /* -------------------------------------------------------------------------- */
    /*                                EXPRESSION                                  */
    /* -------------------------------------------------------------------------- */

    pub fn append_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Int(i) => Value::new_val(i.to_string(), Type::Int),
            Expr::Double(d) => Value::new_val(format!("{:?}", d), Type::Double),
            Expr::Var(name) => {
                let addr = self.lookup_lvalue(name);
                self.rvalue(&addr)
            }
            Expr::BinaryOp { lhs, operator, rhs } => {
                let lhs_val = self.append_expr(lhs);
                let rhs_val = self.append_expr(rhs);
                self.binop(operator.clone(), &lhs_val, &rhs_val)
            }
            Expr::Call { name, args } => self.append_fn_call(name, args),
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

    /* -------------------------------------------------------------------------- */
    /*                                PRINT                                       */
    /* -------------------------------------------------------------------------- */

    fn declare_printf_once(&mut self) {
        if !self.printf_declared {
            let decl = "declare i32 @printf(i8*, ...)\n\n".to_string();
            self.output.insert_str(0, &decl);
            self.printf_declared = true;
        }
    }

    /// Interns the literal as a private global constant and returns a GEP to its first byte,
    /// so repeated uses reuse the same label and the array length stays consistent.
    fn global_str(&mut self, text: &str) -> String {
        if let Some((label, bytes)) = self.string_literals.get(text) {
            return format!(
                "getelementptr inbounds ([{} x i8], [{} x i8]* {}, i32 0, i32 0)",
                bytes, bytes, label
            );
        }

        // Real byte length is the raw UTF-8 size plus the trailing null terminator.
        let bytes = text.as_bytes().len() + 1;

        // Escape characters that would break the LLVM string literal syntax.
        let escaped = text
            .replace('\\', "\\5C")
            .replace('\n', "\\0A")
            .replace('\"', "\\22");

        let label = format!("@.str.{}", self.string_literals.len());
        let global = format!(
            "{} = private constant [{} x i8] c\"{}\\00\"\n",
            label, bytes, escaped
        );
        self.output.insert_str(0, &global);
        self.string_literals
            .insert(text.to_string(), (label.clone(), bytes));

        format!(
            "getelementptr inbounds ([{} x i8], [{} x i8]* {}, i32 0, i32 0)",
            bytes, bytes, label
        )
    }

    pub fn append_print_expr(&mut self, expr: &Expr) {
        self.declare_printf_once();
        let val = self.append_expr(expr);
        let format_ptr = match val.ty {
            Type::Int => self.global_str("%d\n"),
            Type::Double => self.global_str("%f\n"),
            _ => {
                self.error("Unsupported type in println expression");
                return;
            }
        };
        let tmp = self.new_tmp();
        self.append(&format!(
            "{} = call i32 (i8*, ...) @printf(i8* {}, {} {})",
            tmp,
            format_ptr,
            val.ty.llvm(),
            val.repr
        ));
    }

    pub fn append_print_string(&mut self, text: &str) {
        self.declare_printf_once();
        let string_ptr = self.global_str(text);
        let format_ptr = self.global_str("%s\n");
        let tmp = self.new_tmp();
        self.append(&format!(
            "{} = call i32 (i8*, ...) @printf(i8* {}, i8* {})",
            tmp, format_ptr, string_ptr
        ));
    }

    /* -------------------------------------------------------------------------- */
    /*                             FUNCTION DEFINITION                            */
    /* -------------------------------------------------------------------------- */

    pub fn append_fn_def(
        &mut self,
        name: &str,
        params: &[(String, Type)],
        ret_type: &Type,
        body: &[Stmt],
    ) {
        if self.functions.contains_key(name) {
            self.error(&format!("Function '{}' already defined", name));
            return;
        }
        self.functions.insert(
            name.to_string(),
            FnInfo {
                params: params.to_vec(),
                ret_type: ret_type.clone(),
            },
        );

        let params_str = params
            .iter()
            .map(|(n, t)| format!("{} {}", t.llvm(), n))
            .collect::<Vec<_>>()
            .join(", ");

        self.append(&format!(
            "define {} @{}({}) {{",
            ret_type.llvm(),
            name,
            params_str
        ));

        self.enter_scope();

        // insert parameters into scope as local variables
        for (n, t) in params {
            let addr = self.alloca_of_type(n, t);
            self.append(&format!(
                "store {} {}, {}* {}",
                t.llvm(),
                n,
                t.llvm(),
                addr.repr
            ));

            self.current_scope_mut().vars.insert(
                n.clone(),
                ValueObj {
                    name: n.clone(),
                    val: addr,
                    mutable: true,
                },
            );
        }

        for stmt in body {
            self.append_stmt(stmt);
        }

        // If the body doesn't have a "ret", add a default ret
        match ret_type {
            Type::Int => self.append("ret i32 0"),
            Type::Double => self.append("ret double 0.0"),
            Type::Unit => self.append("ret i8 0"),
            _ => self.append("ret i32 0 ; default return"),
        }

        self.leave_scope();
        self.append("}");
        self.newline();
    }

    /* -------------------------------------------------------------------------- */
    /*                               FUNCTION CALL                                */
    /* -------------------------------------------------------------------------- */

    fn append_fn_call(&mut self, name: &str, args: &[Box<Expr>]) -> Value {
        let fn_info = match self.functions.get(name).cloned() {
            Some(params) => params,
            None => {
                self.error(&format!("Call to undefined function '{}'", name));
                return Value::new_val("%undef", Type::Unknown);
            }
        };

        if args.len() != fn_info.params.len() {
            self.error(&format!(
                "Argument count mismatch in call to '{}': expected {}, got {}",
                name,
                fn_info.params.len(),
                args.len()
            ));
            return Value::new_val("%undef", Type::Unknown);
        }

        let mut arg_vals = Vec::new();
        for (arg_expr, (param_name, param_type)) in args.iter().zip(fn_info.params.iter()) {
            let arg_val = self.append_expr(arg_expr);
            if !types_compatible(&arg_val.ty, param_type) {
                self.error(&format!(
                    "Type mismatch in argument '{}' of call to '{}': expected {}, got {}",
                    param_name, name, param_type, arg_val.ty
                ));
                return Value::new_val("%undef", Type::Unknown);
            }
            arg_vals.push(arg_val);
        }

        let tmp = self.new_tmp();
        let args_str = arg_vals
            .iter()
            .map(|v| format!("{} {}", v.ty.llvm(), v.repr))
            .collect::<Vec<_>>()
            .join(", ");

        self.append(&format!(
            "{} = call {} @{}({})",
            tmp,
            fn_info.ret_type.llvm(),
            name,
            args_str
        ));
        Value::new_val(tmp, fn_info.ret_type)
    }
}
