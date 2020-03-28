extern crate phparser;

use phparser::Parser;
use std::path::PathBuf;


#[test]
fn parse_file() {
    let path = "./tests/data/A.php";
    let path = PathBuf::from(path);

    let parser = Parser::new();

    let parsed = parser.parse_file(path);
}