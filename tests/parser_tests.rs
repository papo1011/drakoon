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
        "[Variable { name: \"a\", value: Integer(42) }, Variable { name: \"b\", value: Integer(23) }, Print { value: BinaryOperation { lhs: Variable(\"a\"), operator: Add, rhs: Variable(\"b\") } }]"
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
        "[Variable { name: \"a\", value: Double(42.3) }, Variable { name: \"b\", value: Double(23.7) }, Print { value: BinaryOperation { lhs: Variable(\"a\"), operator: Add, rhs: Variable(\"b\") } }]"
    );
}
