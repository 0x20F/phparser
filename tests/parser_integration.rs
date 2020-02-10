use phparser::Parser;


#[test]
fn test_instance() {
    Parser::new();
}


#[test]
fn test_parse_dir() {
    let parser = Parser::new();

    let files = parser.parse_dir("./tests/data/parse_dir_test");

    assert_eq!(3, files.len());
}