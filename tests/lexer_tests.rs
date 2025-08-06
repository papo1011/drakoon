use drakoon::{lexer::Lexer, tokens::Token};

#[test]
fn test_keywords() {
    let source = "fn return println let const mut while for if else";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.map(|r| r.unwrap().1).collect();

    assert_eq!(
        tokens,
        vec![
            Token::Fn,
            Token::Return,
            Token::Println,
            Token::Let,
            Token::Const,
            Token::Mutable,
            Token::While,
            Token::For,
            Token::If,
            Token::Else,
        ]
    );
}

#[test]
fn test_types() {
    let source = "Int Double";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.map(|r| r.unwrap().1).collect();

    assert_eq!(tokens, vec![Token::IntType, Token::DoubleType]);
}

#[test]
fn test_operators() {
    let source = "+ - * / % == != < <= > >= && || !";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.map(|r| r.unwrap().1).collect();

    assert_eq!(
        tokens,
        vec![
            Token::Plus,
            Token::Minus,
            Token::Mul,
            Token::Div,
            Token::Mod,
            Token::Eq,
            Token::Neq,
            Token::Lt,
            Token::Le,
            Token::Gt,
            Token::Ge,
            Token::And,
            Token::Or,
            Token::Not,
        ]
    );
}

#[test]
fn test_punctuation() {
    let source = ": ; , { } ( ) = -> . :: => @";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.map(|r| r.unwrap().1).collect();

    assert_eq!(
        tokens,
        vec![
            Token::Col,
            Token::Semicolon,
            Token::Comma,
            Token::LBrace,
            Token::RBrace,
            Token::LParen,
            Token::RParen,
            Token::Assign,
            Token::Arrow,
            Token::Dot,
            Token::DoubleColon,
            Token::FatArrow,
            Token::At,
        ]
    );
}

#[test]
fn test_identifiers() {
    let source = "hello world _var var123 _123abc";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.map(|r| r.unwrap().1).collect();

    assert_eq!(
        tokens,
        vec![
            Token::Id("hello".into()),
            Token::Id("world".into()),
            Token::Id("_var".into()),
            Token::Id("var123".into()),
            Token::Id("_123abc".into())
        ]
    );
}

#[test]
fn test_integers() {
    let source = "0 42 123 999";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.map(|r| r.unwrap().1).collect();

    assert_eq!(
        tokens,
        vec![
            Token::Integer(0),
            Token::Integer(42),
            Token::Integer(123),
            Token::Integer(999),
        ]
    );
}

#[test]
fn test_doubles() {
    let source = "3.14 0.5 0.25 42.0 1.5e10 2.3E-5";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.map(|r| r.unwrap().1).collect();

    assert_eq!(
        tokens,
        vec![
            Token::Double(3.14),
            Token::Double(0.5),
            Token::Double(0.25),
            Token::Double(42.0),
            Token::Double(1.5e10),
            Token::Double(2.3E-5)
        ]
    );
}

#[test]
fn test_simple_function() {
    let source = "fn main() { return 42; }";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.map(|r| r.unwrap().1).collect();

    assert_eq!(
        tokens,
        vec![
            Token::Fn,
            Token::Id("main".to_string()),
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Integer(42),
            Token::Semicolon,
            Token::RBrace,
        ]
    );
}

#[test]
fn test_whitespace_skipping() {
    let source = "  fn\t\tmain\n(\n)\n{\n}  ";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.map(|r| r.unwrap().1).collect();

    assert_eq!(
        tokens,
        vec![
            Token::Fn,
            Token::Id("main".to_string()),
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
        ]
    );
}

#[test]
fn test_single_line_comments() {
    let source = r#"
        fn get_num() -> Int // this is a comment
        {
            let x = 42 // another comment
            return x // return value
        }
    "#;
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.map(|r| r.unwrap().1).collect();

    assert_eq!(
        tokens,
        vec![
            Token::Fn,
            Token::Id("get_num".to_string()),
            Token::LParen,
            Token::RParen,
            Token::Arrow,
            Token::IntType,
            Token::LBrace,
            Token::Let,
            Token::Id("x".to_string()),
            Token::Assign,
            Token::Integer(42),
            Token::Return,
            Token::Id("x".to_string()),
            Token::RBrace,
        ]
    );
}

#[test]
fn test_doc_comments() {
    let source = r#"
        ///| This is a function comment
        fn example() -> Unit {
            // comment
            let x = 5
        }
    "#;
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.map(|r| r.unwrap().1).collect();

    assert_eq!(
        tokens,
        vec![
            Token::Fn,
            Token::Id("example".to_string()),
            Token::LParen,
            Token::RParen,
            Token::Arrow,
            Token::UnitType,
            Token::LBrace,
            Token::Let,
            Token::Id("x".to_string()),
            Token::Assign,
            Token::Integer(5),
            Token::RBrace,
        ]
    );
}

#[test]
fn test_token_positions() {
    let source = "fn main";
    let lexer = Lexer::new(source);
    let results: Vec<_> = lexer.collect();

    assert_eq!(results.len(), 2);

    // Check first token (fn)
    let (start, token, end) = results[0].as_ref().unwrap();
    assert_eq!(*token, Token::Fn);
    assert_eq!(*start, 0);
    assert_eq!(*end, 2);

    // Check second token (main)
    let (start, token, end) = results[1].as_ref().unwrap();
    assert_eq!(*token, Token::Id("main".to_string()));
    assert_eq!(*start, 3);
    assert_eq!(*end, 7);
}

#[test]
fn test_multiline_positions() {
    let source = "fn\nmain\n()";
    let lexer = Lexer::new(source);
    let results: Vec<_> = lexer.collect();

    // Check first token: fn
    let (start, token, end) = results[0].as_ref().unwrap();
    assert_eq!(*token, Token::Fn);
    assert_eq!(*start, 0);
    assert_eq!(*end, 2);

    // Check second token: main - on line 2
    let (start, token, end) = results[1].as_ref().unwrap();
    assert_eq!(*token, Token::Id("main".to_string()));
    assert_eq!(*start, 3);
    assert_eq!(*end, 7);

    // Check third token: ( - on line 3
    let (start, token, end) = results[2].as_ref().unwrap();
    assert_eq!(*token, Token::LParen);
    assert_eq!(*start, 8);
    assert_eq!(*end, 9);

    // Check fourth token: ) - on line 3
    let (start, token, end) = results[3].as_ref().unwrap();
    assert_eq!(*token, Token::RParen);
    assert_eq!(*start, 9);
    assert_eq!(*end, 10);
}

#[test]
fn test_complex_expression() {
    let source = "let exp = (x + y) * 2.5";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.map(|r| r.unwrap().1).collect();

    assert_eq!(
        tokens,
        vec![
            Token::Let,
            Token::Id("exp".to_string()),
            Token::Assign,
            Token::LParen,
            Token::Id("x".to_string()),
            Token::Plus,
            Token::Id("y".to_string()),
            Token::RParen,
            Token::Mul,
            Token::Double(2.5),
        ]
    );
}
