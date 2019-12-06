mod function;
mod file;

use function::FunctionModel as Function;
use file::FileModel;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

use walkdir::{WalkDir, DirEntry};



pub async fn run(dirs: Vec<&str>) {
    println!("Indexing the following paths ({:?})", dirs);

    let mut files: HashMap<String, FileModel> = files("/Users/alex.hexan/repo/journal_sys/sys");

    for (_, file) in files.iter_mut() {
        //println!("Found {}", file.filename());
        let functions: Vec<Function> = functions(file).await;
        file.set_functions(functions);

        println!("{}:\nFunctions: {}\n", file.name(), file.functions().len());
    }

    println!("Total files: {}", files.len());
}



/// Get all the files in a given directory, recursively
fn files(dir: &str) -> HashMap<String, FileModel> {

    let mut files: HashMap<String, FileModel> = HashMap::new();
    let walker = WalkDir::new(dir);

    for entry in walker {
        let entry = entry.unwrap();

        if is_php_file(&entry) {
            files.insert(
                entry.path().display().to_string(),
                FileModel::new(entry.path().to_str().unwrap())
            );
        }
    }

    files
}



/// Get all functions in a file
async fn functions(file: &mut FileModel) -> Vec<Function> {

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
                eprintln!("Error: {}; skipped {}", why, file.name());
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
                let function = Function::new(function_data, &file);

                functions.push(function);

                function_data = vec![];
                is_function = false;

                continue;
            }
        }

        // If already in a function, don't try and match aswell
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