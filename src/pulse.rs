mod parser;

fn main() {
    let block = parser::run("This should be an array of directories");

    println!("Parser finished file is: {}", block.filename_parent())
}
