use drakoon::{grammar::ScriptParser, lexer::Lexer};

#[test]
fn expr_int() {
    let mbt = r#"
        let a = 42
        let b = 23
        println(a + b)
    "#;

    let lexer = Lexer::new(mbt);
    let expr = ScriptParser::new().parse(lexer).unwrap();
    assert_eq!(
        &format!("{:?}", expr),
        "[Var { name: \"a\", value: Int(42) }, Var { name: \"b\", value: Int(23) }, Print { value: BinaryOp { lhs: Var(\"a\"), operator: Add, rhs: Var(\"b\") } }]"
    );
}

#[test]
fn expr_double() {
    let mbt = r#"
        let a = 42.3
        let b = 23.7
        println(a + b)
    "#;

    let lexer = Lexer::new(mbt);
    let expr = ScriptParser::new().parse(lexer).unwrap();
    assert_eq!(
        &format!("{:?}", expr),
        "[Var { name: \"a\", value: Double(42.3) }, Var { name: \"b\", value: Double(23.7) }, Print { value: BinaryOp { lhs: Var(\"a\"), operator: Add, rhs: Var(\"b\") } }]"
    );
}
