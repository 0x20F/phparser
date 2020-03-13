use phparser::lexer::{Lexer, Token};
use std::io::BufReader;
use std::fs::File;


// Open a simple file stream for all functions to use
fn setup_stream(path: &str) -> BufReader<File> {
    let f = File::open(path).unwrap();

    BufReader::new(f)
}




#[test]
fn namespace_tokens() {
    let mut stream = setup_stream("./tests/data/lexer_tests/namespace_test/namespace.php");

    let tokens = Lexer::tokenize(&mut stream);

    assert_eq!(1, tokens.len());
}




#[test]
fn import_tokens() {
    let mut stream = setup_stream("./tests/data/lexer_tests/use_test/use.php");

    let tokens = Lexer::tokenize(&mut stream);

    let mut count = 0;

    for token in tokens {
        match token {
            Token::Import(_, _) => count = count + 1,
            _ => continue
        };
    }

    assert_eq!(3, count);
}




#[test]
fn class_tokens() {
    let mut stream = setup_stream("./tests/data/lexer_tests/class_test/class.php");

    let tokens = Lexer::tokenize(&mut stream);

    /*
        ClassStart
        ClassName
        ClassEnd
    */
    assert_eq!(tokens.len(), 18);
}




#[test]
fn function_tokens() {
    let mut stream = setup_stream("./tests/data/lexer_tests/function_test/fun.php");

    let tokens = Lexer::tokenize(&mut stream);

    let expected = 32;
    let mut counter = 0;

    for token in tokens {
        match token {
            Token::FunctionStart(_) |
            Token::FunctionEnd(_) |
            Token::FunctionPrivacy(_, _) |
            Token::FunctionName(_, _) => counter = counter + 1,
            _ => continue
        };
    }

    assert_eq!(expected, counter);
}