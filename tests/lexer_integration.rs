use phparser::lexer::{Lexer};
use phparser::definitions::FileStream;
use std::path::{Path};


// Open a simple file stream for all functions to use
fn setup_stream(path: &str) -> FileStream {
    let p = Path::new(path);
    FileStream::new(&p.to_path_buf())
}


#[test]
fn test_parse_namespace() {
    let namespace = "namespace This\\Is\\a\\PHP\\◊°∆˝¬º\\namespace;";
    let line = 50;

    let parsed = Lexer::parse_namespace(namespace.to_string(), line);

    assert_eq!(parsed.symbol, "This\\Is\\a\\PHP\\◊°∆˝¬º\\namespace");
    assert_eq!(parsed.line, line);
}