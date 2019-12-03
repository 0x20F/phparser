mod blocks;

use blocks::FunctionBlock;


pub fn run(dirs: Vec<&str>) -> FunctionBlock {
    println!("Indexing the following paths ({:?})", dirs);

    FunctionBlock::new(dirs[0])
}


pub fn files() {

}