mod parser;

fn main() {
    let directories = vec!["a", "b", "c"];

    let files = parser::run(directories);
}
