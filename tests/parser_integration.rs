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

    let file_path = "./tests/data/parse_file_test/one.php";
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
    let path = String::from(file_path);
    let file = parser.parse_file(path);

    assert_eq!(file_path, file.path);
    assert_eq!(file_name, file.name);
}