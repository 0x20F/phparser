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


#[test]
fn test_parse_file() {
    let parser = Parser::new();

    // You'll have to assert the count of:
    // classes
    //      functions
    // functions outside of classes
    // namespace
    // no-namespace
    // dependencies
    // dependents

    // For now just check if you get a thing back
    let file_path = String::from("./tests/data/parse_file_test/one.php");
    let file = parser.parse_file(file_path);

    let path = String::from("./tests/data/parse_file_test/one.php");

    assert_eq!(path, file.path);
}