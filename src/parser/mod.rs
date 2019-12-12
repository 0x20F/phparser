mod function;
mod file;
mod class;

use function::FunctionModel as Function;
use file::FileModel;
use class::ClassModel;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, Seek};
use threadpool::ThreadPool;
use std::sync::{ Arc, RwLock };

use walkdir::{ WalkDir, DirEntry };

use paris::Logger;


pub fn run(dirs: Vec<&str>) -> HashMap<String, Arc<RwLock<FileModel>>> {
    println!("Indexing the following paths ({:?})", dirs);

    let mut logger = Logger::new(true);
    let pool = ThreadPool::new(10);
    let mut total_files: HashMap<String, Arc<RwLock<FileModel>>> = HashMap::new();

    logger.info("Starting parser").loading("Parsing files");

    // Find all files in directories
    for dir in &dirs {
        files(dir, &mut total_files);
    }

    // Parse all files in directories
    for (_, file) in total_files.iter() {
        let file = file.clone();

        pool.execute(move || {
            // TODO: Find all classes
            // TODO: Find all functions for that specific class and file (because some files may not have classes)

            let f = file.read().unwrap();
            let functions: Vec<Function> = functions(&f);
            drop(f);

            let mut f = file.write().unwrap();

            f.set_functions(functions);
        });
    }

    pool.join();


    for (namespace, file) in &total_files {
        let mut f = file.write().unwrap();
        f.find_dependencies(&total_files);
        //println!("{} has {} dependencies and {} that depend on it", f.namespace(), f.depends_on.len(), f.depended_by.len());
    }

    logger.success(format!("Parsed: {} files", total_files.len()));


    total_files
}



/// Get all the files in a given directory, recursively
pub fn files(dir: &str, files: &mut HashMap<String, Arc<RwLock<FileModel>>>) {

    let walker = WalkDir::new(dir);
    let mut counter = 0;

    for entry in walker {
        let entry = entry.unwrap();
        counter = counter + 1;

        if is_php_file(&entry) {
            let file = FileModel::new(entry.path().to_str().unwrap());

            let f = file.read().unwrap();
            let name = f.namespace();
            drop(f);

            files.insert(
                name,
                file
            );
        }
    }
}



/*pub fn classes(file: &FileModel) -> Vec<ClassModel> {

}*/



/// Get all functions in a file
pub fn functions(file: &FileModel) -> Vec<Function> {

    let mut functions: Vec<Function> = vec![];

    let file_contents = File::open(file.path()).unwrap();
    let reader = BufReader::new(file_contents);
    
    let mut stack: Vec<i8> = vec![];
    let mut function_data: Vec<String> = vec![];
    let mut is_class = false;
    let mut is_function = false;


    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(why) => {
                //eprintln!("Error: {}; skipped {}", why, file.name());
                break;
            }
        };

        // Simple checks to not waste time
        if line.contains("class") { is_class = true; }
        if !is_class { continue; }


        // Keeping track of code blocks
        if line.contains('{') { stack.push(1); }
        if line.contains('}') {
            stack.pop();

            if stack.len() == 1 && is_function {
                let function = Function::new(function_data, file);

                functions.push(function);

                function_data = vec![];
                is_function = false;

                continue;
            }
        }

        // If already in a function, don't try and match as well
        if stack.len() >= 2 && is_function {
            function_data.push(line);
            continue;
        }

        // Check if it's a function
        if
            line.contains("private function") ||
            line.contains("public function") ||
            line.contains("protected function")
        {
            is_function = true;
            function_data.push(line);
        }
    }

    functions
}



//async fn changed(file: &FileModel) -> Vec<&Function> {}




fn is_php_file(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".php"))
        .unwrap_or(true)
}