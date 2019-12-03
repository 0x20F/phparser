mod parser;

fn main() {
    let directories = vec!["a", "b", "c"];

    let block = parser::run(directories);
}
