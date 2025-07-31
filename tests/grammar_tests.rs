use drakoon::{grammar::ScriptParser, lexer::Lexer};

#[test]
fn calculator4() {
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
