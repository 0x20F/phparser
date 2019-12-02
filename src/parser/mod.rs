mod blocks;

use blocks::FunctionBlock;


pub fn run(dir: &str) -> FunctionBlock {
    println!("Indexing the following paths ({})", dir);

    FunctionBlock::new(dir)
}


pub fn files() {

}