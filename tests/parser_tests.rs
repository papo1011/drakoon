use drakoon::{grammar::ScriptParser, lexer::Lexer};

#[test]
fn expr_int() {
    let mbt = r#"
        let a = 42
        let b = 23
        fn main {
            println(a + b)
        }
    "#;

    let lexer = Lexer::new(mbt);
    let expr = ScriptParser::new().parse(lexer).unwrap();
    assert_eq!(
        &format!("{:?}", expr),
        "[GlobalVarDef { name: \"a\", annot: None, value: Int(42), is_const: false }, GlobalVarDef { name: \"b\", annot: None, value: Int(23), is_const: false }, MainDef { body: [PrintExpr { value: BinaryOp { lhs: Var(\"a\"), operator: Add, rhs: Var(\"b\") } }] }]"
    );
}

#[test]
fn expr_double() {
    let mbt = r#"
        let a = 42.3
        let b = 23.7
        fn main {
            println(a + b)
        }
    "#;

    let lexer = Lexer::new(mbt);
    let expr = ScriptParser::new().parse(lexer).unwrap();
    assert_eq!(
        &format!("{:?}", expr),
        "[GlobalVarDef { name: \"a\", annot: None, value: Double(42.3), is_const: false }, GlobalVarDef { name: \"b\", annot: None, value: Double(23.7), is_const: false }, MainDef { body: [PrintExpr { value: BinaryOp { lhs: Var(\"a\"), operator: Add, rhs: Var(\"b\") } }] }]"
    );
}

#[test]
fn fn_definition_and_return() {
    let mbt = r#"
        fn add(a: Int, b: Int) -> Int {
            return a + b
        }
    "#;

    let lexer = Lexer::new(mbt);
    let expr = ScriptParser::new().parse(lexer).unwrap();

    assert_eq!(
        &format!("{:?}", expr),
        "[FnDef { name: \"add\", params: [(\"a\", Int), (\"b\", Int)], ret_type: Int, body: [Return { value: Some(BinaryOp { lhs: Var(\"a\"), operator: Add, rhs: Var(\"b\") }) }] }]"
    );
}

#[test]
fn fn_call_inside_main() {
    let mbt = r#"
        fn main {
            let x = add(1, 2)
            println(x)
        }
    "#;

    let lexer = Lexer::new(mbt);
    let expr = ScriptParser::new().parse(lexer).unwrap();

    assert_eq!(
        &format!("{:?}", expr),
        "[MainDef { body: [VarDef { name: \"x\", annot: None, value: Call { name: \"add\", args: [Int(1), Int(2)] }, mutable: false }, PrintExpr { value: Var(\"x\") }] }]"
    );
}

#[test]
fn return_constant() {
    let mbt = r#"
        fn constant() -> Int {
            return 42
        }
    "#;

    let lexer = Lexer::new(mbt);
    let expr = ScriptParser::new().parse(lexer).unwrap();

    assert_eq!(
        &format!("{:?}", expr),
        "[FnDef { name: \"constant\", params: [], ret_type: Int, body: [Return { value: Some(Int(42)) }] }]"
    );
}

#[test]
fn multiple_statements_and_return() {
    let mbt = r#"
        fn compute() -> Int {
            let a = 10
            let b = 20
            println("value of a:")
            println(a)
            return a + b
        }
    "#;

    let lexer = Lexer::new(mbt);
    let expr = ScriptParser::new().parse(lexer).unwrap();

    assert_eq!(
        &format!("{:?}", expr),
        "[FnDef { name: \"compute\", params: [], ret_type: Int, body: [VarDef { name: \"a\", annot: None, value: Int(10), mutable: false }, VarDef { name: \"b\", annot: None, value: Int(20), mutable: false }, PrintString { value: \"value of a:\" }, PrintExpr { value: Var(\"a\") }, Return { value: Some(BinaryOp { lhs: Var(\"a\"), operator: Add, rhs: Var(\"b\") }) }] }]"
    );
}
