use drakoon::lexer::Lexer;
use drakoon::tokens::Token;

#[test]
fn test_keywords() {
    let source = "fn return println let const while for if else";
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
    let source = ": ; , { } ( ) = -> . :: =>";
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
        vec![Token::Id, Token::Id, Token::Id, Token::Id, Token::Id,]
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
            Token::Integer,
            Token::Integer,
            Token::Integer,
            Token::Integer,
        ]
    );
}

#[test]
fn test_doubles() {
    let source = "3.14 0.5 .25 42.0 1.5e10 2.3E-5";
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.map(|r| r.unwrap().1).collect();

    assert_eq!(
        tokens,
        vec![
            Token::Double,
            Token::Double,
            Token::Double,
            Token::Double,
            Token::Double,
            Token::Double,
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
            Token::Id,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Integer,
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
            Token::Id,
            Token::Newline,
            Token::LParen,
            Token::Newline,
            Token::RParen,
            Token::Newline,
            Token::LBrace,
            Token::Newline,
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
            Token::Newline,
            Token::Fn,
            Token::Id,
            Token::LParen,
            Token::RParen,
            Token::Arrow,
            Token::IntType,
            Token::Newline,
            Token::LBrace,
            Token::Newline,
            Token::Let,
            Token::Id,
            Token::Assign,
            Token::Integer,
            Token::Newline,
            Token::Return,
            Token::Id,
            Token::Newline,
            Token::RBrace,
            Token::Newline,
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
            Token::Newline,
            Token::Newline,
            Token::Fn,
            Token::Id,
            Token::LParen,
            Token::RParen,
            Token::Arrow,
            Token::UnitType,
            Token::LBrace,
            Token::Newline,
            Token::Newline,
            Token::Let,
            Token::Id,
            Token::Assign,
            Token::Integer,
            Token::Newline,
            Token::RBrace,
            Token::Newline,
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
    assert_eq!(*token, Token::Id);
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

    // Check second token: \n - on line 1
    let (start, token, end) = results[1].as_ref().unwrap();
    assert_eq!(*token, Token::Newline);
    assert_eq!(*start, 2);
    assert_eq!(*end, 3);

    // Check third token: main - on line 2
    let (start, token, end) = results[2].as_ref().unwrap();
    assert_eq!(*token, Token::Id);
    assert_eq!(*start, 3);
    assert_eq!(*end, 7);

    // Check fourth token: \n - on line 2
    let (start, token, end) = results[3].as_ref().unwrap();
    assert_eq!(*token, Token::Newline);
    assert_eq!(*start, 7);
    assert_eq!(*end, 8);

    // Check fifth token: ( - on line 3
    let (start, token, end) = results[4].as_ref().unwrap();
    assert_eq!(*token, Token::LParen);
    assert_eq!(*start, 8);
    assert_eq!(*end, 9);

    // Check sixth token: ) - on line 3
    let (start, token, end) = results[5].as_ref().unwrap();
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
            Token::Id, // exp
            Token::Assign,
            Token::LParen,
            Token::Id, // x
            Token::Plus,
            Token::Id, // y
            Token::RParen,
            Token::Mul,
            Token::Double, // 2.5
        ]
    );
}
