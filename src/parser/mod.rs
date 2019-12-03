mod blocks;

use blocks::FunctionBlock;


pub fn run(dirs: Vec<&str>) -> FunctionBlock {
    println!("Indexing the following paths ({:?})", dirs);

    FunctionBlock::new(dirs[0])
}


/// Get all the files in a given directory, recursively
pub fn files(dir: &str) {}


/// Get all functions in a file
pub fn functions(file: &str) {}