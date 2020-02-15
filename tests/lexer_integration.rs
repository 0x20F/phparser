use phparser::lexer::{Lexer, Tokens};
use phparser::definitions::FileStream;
use std::path::{Path};


// Open a simple file stream for all functions to use
fn setup_stream(path: &str) -> FileStream {
    let p = Path::new(path);
    FileStream::new(&p.to_path_buf())
}





#[test]
fn test_find_namespace() {
    let mut file = setup_stream("./tests/data/lexer_tests/namespace_test/namespace.php");

    let namespace = Lexer::find_namespace(&mut file);

    match namespace {
        Tokens::NotFound() => panic!("Should've found a namespace in namespace.php"),
        _ => ()
    };
}


#[test]
fn test_find_denendencies() {
    let mut file = setup_stream("./tests/data/lexer_tests/dependency_test/use.php");

    let dependencies = Lexer::find_dependencies(&mut file);

    assert_eq!(3, dependencies.len());
}


#[test]
fn test_find_classes() {
    let mut file = setup_stream("./tests/data/lexer_tests/class_test/class.php");

    let classes = Lexer::find_classes(&mut file);

    // Need to account for both classStart and classEnd tokens
    assert_eq!(6 * 2, classes.len());
}