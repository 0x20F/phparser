use phparser::Parser;


#[test]
fn test_instance() {
    Parser::new();
}


#[test]
fn test_parse_dir() {
    let parser = Parser::new();

    // Assert that the amount of files is as expected
    // when things get parsed eventually
    let files = parser.parse_dir("./tests/data/parse_dir_test");

    assert_eq!(3, files.len());
}