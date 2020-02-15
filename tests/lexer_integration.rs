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