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

#[test]
fn if_else_statement() {
    let mbt = r#"
        fn check(value: Int) -> Int {
            if value > 0 {
                println("value is positive")
                return 1
            } else {
                println("value is not positive")
                return 0
            }
        }
    "#;

    let lexer = Lexer::new(mbt);
    let expr = ScriptParser::new().parse(lexer).unwrap();

    assert_eq!(
        &format!("{:?}", expr),
        "[FnDef { name: \"check\", params: [(\"value\", Int)], ret_type: Int, body: [If { cond: BinaryOp { lhs: Var(\"value\"), operator: Gt, rhs: Int(0) }, then_body: [PrintString { value: \"value is positive\" }, Return { value: Some(Int(1)) }], else_body: Some([PrintString { value: \"value is not positive\" }, Return { value: Some(Int(0)) }]) }] }]"
    );
}

#[test]
fn if_statement_without_else() {
    let mbt = r#"
        fn check(value: Int) -> Int {
            if value > 0 {
                println("value is positive")
                return 1
            }
            return 0
        }
    "#;
    let lexer = Lexer::new(mbt);
    let expr = ScriptParser::new().parse(lexer).unwrap();

    assert_eq!(
        &format!("{:?}", expr),
        "[FnDef { name: \"check\", params: [(\"value\", Int)], ret_type: Int, body: [If { cond: BinaryOp { lhs: Var(\"value\"), operator: Gt, rhs: Int(0) }, then_body: [PrintString { value: \"value is positive\" }, Return { value: Some(Int(1)) }], else_body: None }, Return { value: Some(Int(0)) }] }]"
    );
}

#[test]
fn while_simple_increment() {
    let mbt = r#"
        fn inc_until(n: Int) -> Int {
            let mut i = 0
            while i < n {
                i = i + 1
            }
            return i
        }
    "#;

    let lexer = Lexer::new(mbt);
    let ast = ScriptParser::new().parse(lexer).unwrap();

    assert_eq!(
        &format!("{:?}", ast),
        "[FnDef { name: \"inc_until\", params: [(\"n\", Int)], ret_type: Int, body: [VarDef { name: \"i\", annot: None, value: Int(0), mutable: true }, While { cond: BinaryOp { lhs: Var(\"i\"), operator: Lt, rhs: Var(\"n\") }, body: [VarAssign { name: \"i\", value: BinaryOp { lhs: Var(\"i\"), operator: Add, rhs: Int(1) } }] }, Return { value: Some(Var(\"i\")) }] }]"
    );
}

#[test]
fn for_full_init_cond_step() {
    let mbt = r#"
        fn sum_to(n: Int) -> Int {
            let mut s = 0
            for i = 0; i < n; i = i + 1 {
                s = s + i
            }
            return s
        }
    "#;

    let lexer = Lexer::new(mbt);
    let ast = ScriptParser::new().parse(lexer).unwrap();

    assert_eq!(
        &format!("{:?}", ast),
        "[FnDef { name: \"sum_to\", params: [(\"n\", Int)], ret_type: Int, body: [VarDef { name: \"s\", annot: None, value: Int(0), mutable: true }, For { init: Some(VarDef { name: \"i\", annot: None, value: Int(0), mutable: true }), cond: Some(BinaryOp { lhs: Var(\"i\"), operator: Lt, rhs: Var(\"n\") }), step: Some(VarAssign { name: \"i\", value: BinaryOp { lhs: Var(\"i\"), operator: Add, rhs: Int(1) } }), body: [VarAssign { name: \"s\", value: BinaryOp { lhs: Var(\"s\"), operator: Add, rhs: Var(\"i\") } }] }, Return { value: Some(Var(\"s\")) }] }]"
    );
}

#[test]
fn for_without_cond_infinite() {
    let mbt = r#"
        fn first_over_three() -> Int {
            let mut i = 0
            for i = 0; ; i = i + 1 {
                if i > 3 {
                    return i
                }
            }
        }
    "#;

    let lexer = Lexer::new(mbt);
    let ast = ScriptParser::new().parse(lexer).unwrap();

    assert_eq!(
        &format!("{:?}", ast),
        "[FnDef { name: \"first_over_three\", params: [], ret_type: Int, body: [VarDef { name: \"i\", annot: None, value: Int(0), mutable: true }, For { init: Some(VarDef { name: \"i\", annot: None, value: Int(0), mutable: true }), cond: None, step: Some(VarAssign { name: \"i\", value: BinaryOp { lhs: Var(\"i\"), operator: Add, rhs: Int(1) } }), body: [If { cond: BinaryOp { lhs: Var(\"i\"), operator: Gt, rhs: Int(3) }, then_body: [Return { value: Some(Var(\"i\")) }], else_body: None }] }] }]"
    );
}

#[test]
fn for_only_cond() {
    let mbt = r#"
        fn count_up_to_ten() -> Int {
            let mut i = 0
            for ; i < 10; {
                i = i + 1
            }
            return i
        }
    "#;

    let lexer = Lexer::new(mbt);
    let ast = ScriptParser::new().parse(lexer).unwrap();

    assert_eq!(
        &format!("{:?}", ast),
        "[FnDef { name: \"count_up_to_ten\", params: [], ret_type: Int, body: [VarDef { name: \"i\", annot: None, value: Int(0), mutable: true }, For { init: None, cond: Some(BinaryOp { lhs: Var(\"i\"), operator: Lt, rhs: Int(10) }), step: None, body: [VarAssign { name: \"i\", value: BinaryOp { lhs: Var(\"i\"), operator: Add, rhs: Int(1) } }] }, Return { value: Some(Var(\"i\")) }] }]"
    );
}

#[test]
fn nested_while_inside_for() {
    let mbt = r#"
        fn nested() -> Unit {
            let mut i = 0
            for i = 0; i < 2; i = i + 1 {
                while i < 2 {
                    println(i)
                    i = i + 1
                }
            }
        }
    "#;

    let lexer = Lexer::new(mbt);
    let ast = ScriptParser::new().parse(lexer).unwrap();

    assert_eq!(
        &format!("{:?}", ast),
        "[FnDef { name: \"nested\", params: [], ret_type: Unit, body: [VarDef { name: \"i\", annot: None, value: Int(0), mutable: true }, For { init: Some(VarDef { name: \"i\", annot: None, value: Int(0), mutable: true }), cond: Some(BinaryOp { lhs: Var(\"i\"), operator: Lt, rhs: Int(2) }), step: Some(VarAssign { name: \"i\", value: BinaryOp { lhs: Var(\"i\"), operator: Add, rhs: Int(1) } }), body: [While { cond: BinaryOp { lhs: Var(\"i\"), operator: Lt, rhs: Int(2) }, body: [PrintExpr { value: Var(\"i\") }, VarAssign { name: \"i\", value: BinaryOp { lhs: Var(\"i\"), operator: Add, rhs: Int(1) } }] }] }] }]"
    );
}

#[test]
fn fixed_array_definition() {
    let mbt = r#"
        fn main {
            let arr : FixedArray[Int] = [1, 2, 3, 4, 5]
            println("Array defined")
        }
    "#;

    let lexer = Lexer::new(mbt);
    let ast = ScriptParser::new().parse(lexer).unwrap();

    assert_eq!(
        &format!("{:?}", ast),
        "[MainDef { body: [FixedArrayDef { name: \"arr\", annot: FixedArray(Int, Some(5)), values: [Int(1), Int(2), Int(3), Int(4), Int(5)], mutable: false }, PrintString { value: \"Array defined\" }] }]"
    );
}

#[test]
fn read_index_array() {
    let mbt = r#"
        fn main {
            let arr : FixedArray[Int] = [10, 20, 30]
            let value = arr[1]
            println(value)
        }
    "#;

    let lexer = Lexer::new(mbt);
    let ast = ScriptParser::new().parse(lexer).unwrap();

    assert_eq!(
        &format!("{:?}", ast),
        "[MainDef { body: [FixedArrayDef { name: \"arr\", annot: FixedArray(Int, Some(3)), values: [Int(10), Int(20), Int(30)], mutable: false }, VarDef { name: \"value\", annot: None, value: Index { name: \"arr\", index: Int(1) }, mutable: false }, PrintExpr { value: Var(\"value\") }] }]"
    );
}
