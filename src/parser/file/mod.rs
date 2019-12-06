use super::function::FunctionModel;

// TODO: These shouldn't all be strings
pub struct FileModel {
    name: String,
    path: String,
    ext: String,
    functions: Vec<FunctionModel>
}


impl FileModel {

    pub fn new(path: &str) -> FileModel {
        let mut vec: Vec<&str> = path.split(&['/', '.', '\\'][..]).collect();

        let ext = String::from(vec.pop().unwrap());
        let name = String::from(vec.pop().unwrap());
        let path = String::from(path);

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

    pub fn update_function(&mut self, function: FunctionModel) {
        todo!();
    }

    pub fn set_functions(&mut self, functions: Vec<FunctionModel>) {
        self.functions = functions;
    }




    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn functions(&self) -> &Vec<FunctionModel> {
        &self.functions
    }
}