mod function;
mod file;

use function::Function;



pub fn run(dirs: Vec<&str>) -> Function {
    println!("Indexing the following paths ({:?})", dirs);

    Function::new(dirs[0])
}


/// Get all the files in a given directory, recursively
pub fn files(dir: &str) {}


/// Get all functions in a file
pub fn functions(file: &str) {}