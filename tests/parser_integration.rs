use phparser::Parser;
use std::path::Path;


#[test]
fn instance() {
    Parser::new();
}


#[test]
fn parse_dir() {
    let parser = Parser::new();

    let path = Path::new("./tests/data/parse_dir_test");
    let files = parser.parse_dir(&path);

    assert_eq!(3, files.len());
}


#[test]
fn parse_file() {
    let parser = Parser::new();

    let file_path = Path::new("./tests/data/parse_file_test/one.php");
    let file_name = "one.php";

    // You'll have to assert the count of:
    // classes
    //      functions
    // functions outside of classes
    // namespace
    // no-namespace
    // dependencies
    // dependents

    // For now just check if you get a thing back
    let file = parser.parse_file(file_path.to_path_buf());

    assert_eq!(file_path, file.path);
    assert_eq!(file_name, file.name);
}