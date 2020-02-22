use phparser::lexer::{Lexer, Token};
use phparser::definitions::FileStream;
use std::path::{Path};


// Open a simple file stream for all functions to use
fn setup_stream(path: &str) -> FileStream {
    let p = Path::new(path);
    FileStream::new(&p.to_path_buf())
}




#[test]
fn namespace_tokens() {
    let mut stream = setup_stream("./tests/data/lexer_tests/namespace_test/namespace.php");

    let tokens = Lexer::tokenize(&mut stream);

    assert_eq!(1, tokens.len());
}




#[test]
fn use_tokens() {
    let mut stream = setup_stream("./tests/data/lexer_tests/use_test/use.php");

    let tokens = Lexer::tokenize(&mut stream);

    let mut count = 0;

    for token in tokens {
        match token {
            Token::Use(_) => count = count + 1,
            _ => continue
        };
    }

    assert_eq!(3, count);
}




#[test]
fn class_tokens() {
    let mut stream = setup_stream("./tests/data/lexer_tests/class_test/class.php");

    let tokens = Lexer::tokenize(&mut stream);

    assert_eq!(6 * 2, tokens.len());
}




#[test]
fn function_tokens() {
    let mut stream = setup_stream("./tests/data/lexer_tests/function_test/fun.php");

    let tokens = Lexer::tokenize(&mut stream);

    let expected = 8;
    let mut counter = 0;

    for token in tokens {
        match token {
            Token::Method(_, _) => counter = counter + 1,
            Token::Function(_, _) => counter = counter + 1,
            _ => continue
        };
    }

    assert_eq!(expected, counter);
}