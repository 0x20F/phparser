use phparser::lexer::{Lexer, TokenType};
use phparser::definitions::FileStream;
use std::path::{Path};


// Open a simple file stream for all functions to use
fn setup_stream(path: &str) -> FileStream {
    let p = Path::new(path);
    FileStream::new(&p.to_path_buf())
}




#[test]
fn test_class_tokens() {
    let mut stream = setup_stream("./tests/data/lexer_tests/class_test/class.php");

    let tokens = Lexer::tokenize(&mut stream);

    assert_eq!(6 * 2, tokens.len());
}




#[test]
fn test_function_tokens() {
    let mut stream = setup_stream("./tests/data/lexer_tests/function_test/fun.php");

    let tokens = Lexer::tokenize(&mut stream);

    let expected = 8;
    let mut counter = 0;

    for token in tokens {
        match token.token {
            TokenType::MethodStart => counter = counter + 1,
            TokenType::FunctionStart => counter = counter + 1,
            _ => continue
        };
    }

    assert_eq!(expected, counter);
}