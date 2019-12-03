use super::function::FunctionModel;

// TODO: These shouldn't all be strings
pub struct FileModel {
    pub name: String,
    pub ext: String,
    pub path: String,
    pub functions: Vec<FunctionModel>
}


impl FileModel {

    pub fn new(path: &str) -> FileModel {
        let mut vec: Vec<&str> = path.split(&['/', '.', '\\'][..]).collect();

        let ext = String::from(vec.pop().unwrap());
        let name = String::from(vec.pop().unwrap());
        let path = String::from(path);

        println!("ext   : {}\nname  : {}\npath  : {}\n\n", ext, name, path);

        FileModel {
            name,
            ext,
            path,
            functions: vec![]
        }
    }

    pub fn add_function(&mut self, function: FunctionModel) {
        self.functions.push(function);
    }
}