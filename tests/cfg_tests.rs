use drakoon::ast::{Expr, Stmt};
use drakoon::cfg::{CfgNodeKind, build_cfg_from_stmts};

#[test]
fn straight_line_cfg() {
    let stmts = vec![
        Stmt::VarAssign {
            name: "x".into(),
            value: Box::new(Expr::Int(1)),
        },
        Stmt::VarAssign {
            name: "y".into(),
            value: Box::new(Expr::Int(2)),
        },
        Stmt::Return {
            value: Some(Box::new(Expr::Int(3))),
        },
    ];

    let g = build_cfg_from_stmts(&stmts);
    assert_eq!(g.nodes.len(), 5); // Entry, Exit, x=1, y=2, return 3

    let entry = g.entry;
    let exit = g.exit;

    let n_x = g.nodes[entry].next.unwrap();
    match &g.nodes[n_x].kind {
        CfgNodeKind::Assign { name, .. } => assert_eq!(name, "x"),
        _ => panic!("expected assign x"),
    };

    let n_y = g.nodes[n_x].next.unwrap();
    match &g.nodes[n_y].kind {
        CfgNodeKind::Assign { name, .. } => assert_eq!(name, "y"),
        _ => panic!("expected assign y"),
    };

    let n_ret = g.nodes[n_y].next.unwrap();
    match &g.nodes[n_ret].kind {
        CfgNodeKind::Return { .. } => {}
        _ => panic!("expected return"),
    };

    assert_eq!(g.nodes[n_ret].next, Some(exit));
    assert!(g.nodes[exit].preds.contains(&n_ret));
}

#[test]
fn if_cfg() {
    let stmts = vec![
        Stmt::If {
            cond: Box::new(Expr::Bool(true)),
            then_body: vec![Stmt::VarAssign {
                name: "x".into(),
                value: Box::new(Expr::Int(1)),
            }],
            else_body: Some(vec![Stmt::VarAssign {
                name: "x".into(),
                value: Box::new(Expr::Int(2)),
            }]),
        },
        Stmt::Return { value: None },
    ];

    let g = build_cfg_from_stmts(&stmts);
    let entry = g.entry;
    let exit = g.exit;

    let cond_id = g.nodes[entry].next.expect("entry should point to cond");
    match &g.nodes[cond_id].kind {
        CfgNodeKind::Condition { .. } => {}
        _ => panic!("expected condition"),
    };

    let t = g.nodes[cond_id].true_next.expect("cond true next");
    let f = g.nodes[cond_id].false_next.expect("cond false next");

    // both branches should eventually point to return then exit
    let t_next = g.nodes[t].next.expect("then body next");
    let f_next = g.nodes[f].next.expect("else body next");

    match &g.nodes[t].kind {
        CfgNodeKind::Assign { .. } => {}
        _ => panic!("expected then assign"),
    };
    match &g.nodes[f].kind {
        CfgNodeKind::Assign { .. } => {}
        _ => panic!("expected else assign"),
    };

    // The next after both branches is the return node
    assert_eq!(t_next, f_next);
    match &g.nodes[t_next].kind {
        CfgNodeKind::Return { .. } => {}
        _ => panic!("expected return after if"),
    };
    assert_eq!(g.nodes[t_next].next, Some(exit));
}

#[test]
fn while_cfg() {
    let stmts = vec![
        Stmt::While {
            cond: Box::new(Expr::Bool(true)),
            body: vec![Stmt::VarAssign {
                name: "x".into(),
                value: Box::new(Expr::Int(1)),
            }],
        },
        Stmt::Return { value: None },
    ];

    let g = build_cfg_from_stmts(&stmts);
    let entry = g.entry;
    let exit = g.exit;

    let cond_id = g.nodes[entry].next.expect("entry should point to cond");
    match &g.nodes[cond_id].kind {
        CfgNodeKind::Condition { .. } => {}
        _ => panic!("expected condition"),
    };

    let body_id = g.nodes[cond_id].true_next.expect("true next to body");
    match &g.nodes[body_id].kind {
        CfgNodeKind::Assign { .. } => {}
        _ => panic!("expected body assign"),
    };

    // Body should go back to condition
    assert_eq!(g.nodes[body_id].next, Some(cond_id));

    // False should go to the return, which goes to exit
    let after_loop = g.nodes[cond_id].false_next.expect("false next after loop");
    match &g.nodes[after_loop].kind {
        CfgNodeKind::Return { .. } => {}
        _ => panic!("expected return after loop"),
    };
    assert_eq!(g.nodes[after_loop].next, Some(exit));
}
