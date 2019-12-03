mod function;
mod file;

use function::FunctionModel as Function;
use file::FileModel as File;
use walkdir::{WalkDir, DirEntry};



pub fn run(dirs: Vec<&str>) {
    println!("Indexing the following paths ({:?})", dirs);

    // TODO: For each FileModel in the vector, find functions for it and all that
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
pub fn files(dir: &str) -> Vec<FileModel> {

    let mut files: Vec<FileModel> = vec![];
    let walker = WalkDir::new(dir);

    for entry in walker {
        let entry = entry.unwrap();

        if is_php_file(&entry) {
            // TODO: Return FileModels and return a vector of them
            println!("{}", entry.path().display());
            files.push(File::new(entry.path().to_str().unwrap()));
        }
    }

    files
}



/// Get all functions in a file
pub fn functions(file: &str) {}