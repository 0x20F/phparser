mod function;
mod file;

use function::FunctionModel as Function;
use walkdir::{WalkDir, DirEntry};



pub fn run(dirs: Vec<&str>) {
    println!("Indexing the following paths ({:?})", dirs);

    files("/Users/alex.hexan/repo/journal_sys/tests/unit");
}



fn is_php_file(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".php"))
        .unwrap_or(true)
}



/// Get all the files in a given directory, recursively
pub fn files(dir: &str) {

    let walker = WalkDir::new(dir);

    for entry in walker {
        let entry = entry.unwrap();

        if is_php_file(&entry) {
            println!("{}", entry.path().display());
        }
    }
}



/// Get all functions in a file
pub fn functions(file: &str) {}