use drakoon::lexer::Lexer;
use drakoon::token::Token;

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
            Token::Id,
            Token::LParen,
            Token::RParen,
            Token::Arrow,
            Token::IntType,
            Token::LBrace,
            Token::Let,
            Token::Id,
            Token::Assign,
            Token::Integer,
            Token::Return,
            Token::Id,
            Token::RBrace,
        ]
    );
}

#[test]
fn test_doc_comments() {
    let source = r#"
        ///| This is a function comment
        fn main() {
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
            Token::Id,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::Let,
            Token::Id,
            Token::Assign,
            Token::Integer,
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
    assert_eq!(*token, Token::Id);
    assert_eq!(*start, 3);
    assert_eq!(*end, 7);
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
