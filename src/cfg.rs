use std::collections::HashMap;

use crate::ast::{Expr, Stmt};
use crate::types::Type;

pub type NodeId = usize;

#[derive(Clone, Debug, PartialEq)]
pub enum CfgNodeKind {
    Entry,
    Exit,
    Assign {
        name: String,
        value: Expr,
    },
    VarDef {
        name: String,
        annot: Option<Type>,
        value: Expr,
        mutable: bool,
    },
    GlobalVarDef {
        name: String,
        annot: Option<Type>,
        value: Expr,
        is_const: bool,
    },
    FixedArrayDef {
        name: String,
        annot: Type,
        values: Vec<Expr>,
        mutable: bool,
    },
    PrintExpr {
        value: Expr,
    },
    PrintString {
        value: String,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Return {
        value: Option<Expr>,
    },
    Condition {
        cond: Expr,
    },
}

#[derive(Clone, Debug)]
pub struct CfgNode {
    pub id: NodeId,
    pub kind: CfgNodeKind,
    pub next: Option<NodeId>,
    pub true_next: Option<NodeId>,
    pub false_next: Option<NodeId>,
    pub preds: Vec<NodeId>,
}

impl CfgNode {
    fn new(id: NodeId, kind: CfgNodeKind) -> Self {
        Self {
            id,
            kind,
            next: None,
            true_next: None,
            false_next: None,
            preds: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cfg {
    pub nodes: Vec<CfgNode>,
    pub entry: NodeId,
    pub exit: NodeId,
}

impl Cfg {
    pub fn new() -> Self {
        let mut nodes = Vec::new();
        let entry = 0usize;
        nodes.push(CfgNode::new(entry, CfgNodeKind::Entry));
        let exit = 1usize;
        nodes.push(CfgNode::new(exit, CfgNodeKind::Exit));
        Self { nodes, entry, exit }
    }

    pub fn add_node(&mut self, kind: CfgNodeKind) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(CfgNode::new(id, kind));
        id
    }

    fn add_pred(&mut self, to: NodeId, pred: NodeId) {
        if to < self.nodes.len() {
            self.nodes[to].preds.push(pred);
        }
    }

    fn set_next(&mut self, from: NodeId, to: NodeId) {
        self.nodes[from].next = Some(to);
        self.add_pred(to, from);
    }

    fn set_true(&mut self, from: NodeId, to: NodeId) {
        self.nodes[from].true_next = Some(to);
        self.add_pred(to, from);
    }

    fn set_false(&mut self, from: NodeId, to: NodeId) {
        self.nodes[from].false_next = Some(to);
        self.add_pred(to, from);
    }

    /// Export a Graphviz DOT representation of the CFG
    pub fn to_dot(&self, name: &str) -> String {
        let mut out = String::new();
        out.push_str(&format!("digraph {} {{\n", name));
        out.push_str("  node [shape=box];\n");
        for n in &self.nodes {
            let label = match &n.kind {
                CfgNodeKind::Entry => "Entry".to_string(),
                CfgNodeKind::Exit => "Exit".to_string(),
                CfgNodeKind::Assign { name, value } => format!("{} = {:?}", name, value),
                CfgNodeKind::VarDef {
                    name,
                    annot,
                    value,
                    mutable,
                } => {
                    let m = if *mutable { "mut " } else { "" };
                    let a = annot
                        .as_ref()
                        .map(|t| format!(": {:?}", t))
                        .unwrap_or_default();
                    format!("let {}{}{} = {:?}", m, name, a, value)
                }
                CfgNodeKind::GlobalVarDef {
                    name,
                    annot,
                    value,
                    is_const,
                } => {
                    let m = if *is_const { "const " } else { "var " };
                    let a = annot
                        .as_ref()
                        .map(|t| format!(": {:?}", t))
                        .unwrap_or_default();
                    format!("{}{}{} = {:?}", m, name, a, value)
                }
                CfgNodeKind::FixedArrayDef {
                    name,
                    annot,
                    values,
                    mutable,
                } => {
                    let m = if *mutable { "mut " } else { "" };
                    format!("let {}{}: {:?} = {:?}", m, name, annot, values)
                }
                CfgNodeKind::PrintExpr { value } => format!("print {:?}", value),
                CfgNodeKind::PrintString { value } => format!("print \"{}\"", value),
                CfgNodeKind::Call { name, args } => format!("call {}({:?})", name, args),
                CfgNodeKind::Return { value } => match value {
                    Some(v) => format!("return {:?}", v),
                    None => "return".to_string(),
                },
                CfgNodeKind::Condition { cond } => format!("if {:?}", cond),
            };
            out.push_str(&format!(
                "  {} [label=\"{}\"];\n",
                n.id,
                label.replace('"', "\\\"")
            ));
        }
        for n in &self.nodes {
            if let Some(to) = n.next {
                out.push_str(&format!("  {} -> {};\n", n.id, to));
            }
            if let Some(to) = n.true_next {
                out.push_str(&format!("  {} -> {} [label=\"T\"];\n", n.id, to));
            }
            if let Some(to) = n.false_next {
                out.push_str(&format!("  {} -> {} [label=\"F\"];\n", n.id, to));
            }
        }
        out.push_str("}\n");
        out
    }
}

impl Default for Cfg {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ProgramCfg {
    pub main: Option<Cfg>,
    pub functions: HashMap<String, Cfg>,
}

impl ProgramCfg {
    pub fn new() -> Self {
        Self {
            main: None,
            functions: HashMap::new(),
        }
    }
}

impl Default for ProgramCfg {
    fn default() -> Self {
        Self::new()
    }
}

struct CfgBuilder {
    g: Cfg,
}

impl CfgBuilder {
    fn new() -> Self {
        Self { g: Cfg::new() }
    }

    fn build_block(&mut self, stmts: &[Stmt], next: NodeId) -> NodeId {
        let mut acc = next;
        for s in stmts.iter().rev() {
            acc = self.build_stmt(s, acc);
        }
        acc
    }

    fn build_stmt(&mut self, stmt: &Stmt, next: NodeId) -> NodeId {
        match stmt {
            Stmt::MainDef { body } => {
                // Treat as a block; caller should normally start from body
                self.build_block(body, next)
            }
            Stmt::GlobalVarDef {
                name,
                annot,
                value,
                is_const,
            } => {
                let id = self.g.add_node(CfgNodeKind::GlobalVarDef {
                    name: name.clone(),
                    annot: annot.clone(),
                    value: *value.clone(),
                    is_const: *is_const,
                });
                self.g.set_next(id, next);
                id
            }
            Stmt::VarDef {
                name,
                annot,
                value,
                mutable,
            } => {
                let id = self.g.add_node(CfgNodeKind::VarDef {
                    name: name.clone(),
                    annot: annot.clone(),
                    value: *value.clone(),
                    mutable: *mutable,
                });
                self.g.set_next(id, next);
                id
            }
            Stmt::VarAssign { name, value } => {
                let id = self.g.add_node(CfgNodeKind::Assign {
                    name: name.clone(),
                    value: *value.clone(),
                });
                self.g.set_next(id, next);
                id
            }
            Stmt::FixedArrayDef {
                name,
                annot,
                values,
                mutable,
            } => {
                let id = self.g.add_node(CfgNodeKind::FixedArrayDef {
                    name: name.clone(),
                    annot: annot.clone(),
                    values: values.clone(),
                    mutable: *mutable,
                });
                self.g.set_next(id, next);
                id
            }
            Stmt::PrintExpr { value } => {
                let id = self.g.add_node(CfgNodeKind::PrintExpr {
                    value: *value.clone(),
                });
                self.g.set_next(id, next);
                id
            }
            Stmt::PrintString { value } => {
                let id = self.g.add_node(CfgNodeKind::PrintString {
                    value: value.clone(),
                });
                self.g.set_next(id, next);
                id
            }
            Stmt::FnDef {
                name: _,
                params: _,
                ret_type: _,
                body,
            } => self.build_block(body, next),
            Stmt::FnCall { name, args } => {
                let id = self.g.add_node(CfgNodeKind::Call {
                    name: name.clone(),
                    args: args.clone(),
                });
                self.g.set_next(id, next);
                id
            }
            Stmt::Return { value } => {
                let id = self.g.add_node(CfgNodeKind::Return {
                    value: value.as_ref().map(|v| *v.clone()),
                });
                self.g.set_next(id, self.g.exit);
                id
            }
            Stmt::If {
                cond,
                then_body,
                else_body,
            } => {
                let else_entry = if let Some(else_block) = else_body {
                    self.build_block(else_block, next)
                } else {
                    next
                };
                let then_entry = self.build_block(then_body, next);
                let cond_id = self.g.add_node(CfgNodeKind::Condition {
                    cond: *cond.clone(),
                });
                self.g.set_true(cond_id, then_entry);
                self.g.set_false(cond_id, else_entry);
                cond_id
            }
            Stmt::While { cond, body } => {
                let cond_id = self.g.add_node(CfgNodeKind::Condition {
                    cond: *cond.clone(),
                });
                // Body loops back to condition
                let body_entry = self.build_block(body, cond_id);
                self.g.set_true(cond_id, body_entry);
                // While false goes to next after the loop
                self.g.set_false(cond_id, next);
                cond_id
            }
            Stmt::For {
                init,
                cond,
                step,
                body,
            } => {
                // for (init; cond; step) { body } -> init; while (cond) { body; step }
                // Condition defaults to 'true' if None
                let cond_expr = cond
                    .as_ref()
                    .map(|e| *e.clone())
                    .unwrap_or(Expr::Bool(true));
                let cond_id = self.g.add_node(CfgNodeKind::Condition { cond: cond_expr });

                // Build step then body so that tail of body points to step and step points to cond
                let after_body = if let Some(step_stmt) = step {
                    let step_entry = self.build_stmt(step_stmt, cond_id);
                    // body goes to step
                    self.build_block(body, step_entry)
                } else {
                    // body goes back to cond directly
                    self.build_block(body, cond_id)
                };

                self.g.set_true(cond_id, after_body);
                self.g.set_false(cond_id, next);

                if let Some(init_stmt) = init {
                    // init executes before loop, then jumps to cond
                    self.build_stmt(init_stmt, cond_id)
                } else {
                    cond_id
                }
            }
        }
    }
}

pub fn build_cfg_from_stmts(stmts: &[Stmt]) -> Cfg {
    let mut b = CfgBuilder::new();
    // Body flows to Exit by default at the end
    let entry_block = b.build_block(stmts, b.g.exit);
    // Connect Entry to the first statement (or Exit if empty)
    b.g.set_next(b.g.entry, entry_block);
    b.g
}

pub fn build_program_cfg(program: &[Stmt]) -> ProgramCfg {
    let mut prog = ProgramCfg::new();
    for s in program {
        match s {
            Stmt::MainDef { body } => {
                prog.main = Some(build_cfg_from_stmts(body));
            }
            Stmt::FnDef { name, body, .. } => {
                prog.functions
                    .insert(name.clone(), build_cfg_from_stmts(body));
            }
            _ => {
                // Non-function top-level statements could be handled as a program init CFG in the future
            }
        }
    }
    prog
}
