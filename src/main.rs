mod parser;

fn main() {
    let directories = vec!["a", "b", "c"];

    let block = parser::run(directories);

    println!("Parser finished file is: {}", block.filename_parent())
}
