use super::function::FunctionModel;
use super::class::ClassModel;

use std::fs::{ File, read_to_string };
use std::io::{ Cursor, BufReader, BufRead };
use std::collections::HashMap;
use std::sync::{ Arc, RwLock };
use std::rc::Rc;


// TODO: These shouldn't all be strings
pub struct FileModel {
    name    : String,
    path    : String,
    ext     : String,
    pub contents: String,

    functions           : Option<Vec<FunctionModel>>, // Functions that don't belong to a class
    // All methods belonging to classes will be in their respective objects
    classes             : Option<Vec<ClassModel>>,
    namespace           : String,
    dependencies        : Option<Vec<String>>,

    // TODO: Temporary pubs
    // TODO: Use the class models in here instead?
    pub depends_on     : Vec<Arc<RwLock<FileModel>>>,
    pub depended_by    : Vec<Arc<RwLock<FileModel>>>
}


impl FileModel {

    pub fn new(path: &str) -> Arc<RwLock<FileModel>> {
        // TODO: Change name
        let mut vec: Vec<&str> = path.split(&['/', '\\'][..]).collect();

        let content = read_to_string(path).unwrap();

        // TODO: Use the cursor to keep track of line positions
        let reader = Cursor::new(&content);

        let mut dependencies: Vec<String> = vec![];
        let mut namespace: Option<String> = None;

        for line in reader.clone().lines() {
            let line = match line {
                Ok(line) => line,
                Err(why) => {
                    break;
                }
            };

            if line.contains("namespace") {
                namespace = Some(line.clone());
            }

            if line.starts_with("use") {
                let space = line.split(&[' ', ';'][..])
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .trim_start_matches("\\")
                    .to_string();

                dependencies.push(space);
            }

            // Dependencies and namespaces should be declared before
            // class initialisation. If we hit class, stop searching.
            if line.contains("class") {
                break;
            }
        }


        let file: Vec<&str> = vec.last().unwrap().split(".").collect();

        // filename.php || filename.class.php || filename.random.lmao.php
        let name = file.first().unwrap().to_string();
        let ext = file.last().unwrap().to_string();

        let path = String::from(path);

        let namespace = match namespace {
            Some(space) => {
                let mut value = space.split(&[' ', ';'][..])
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .to_string();

                let value = value.trim_start_matches("\\");

                // TODO: Use class name instead since that's how they're defined and classes don't always match file names
                format!("{}\\{}", value, name)
            },
            None => name.clone()
        };



        Arc::new(RwLock::new(FileModel {
            name,
            ext,
            path,
            contents: content,
            functions: None,
            classes: None,
            namespace,
            dependencies: Some(dependencies),
            depends_on: vec![],
            depended_by: vec![]
        }))
    }



    pub fn add_function(&mut self, function: FunctionModel) {
        let mut functions = match self.functions.take() {
            Some(fns) => fns,
            None => vec![]
        };

        functions.push(function);
        self.functions = Some(functions);
    }

    pub fn update_function(&mut self, function: FunctionModel) {
        todo!();
    }

    pub fn set_functions(&mut self, functions: Vec<FunctionModel>) {
        self.functions = Some(functions);
    }



    pub fn find_dependencies(&mut self, index: &HashMap<String, Arc<RwLock<FileModel>>>) {
        let deps = match self.dependencies.take() {
            Some(deps) => deps,
            None => return
        };

        let mut refs: Vec<Arc<RwLock<FileModel>>> = vec![];

        for dep in &deps  {
            let file = index.get(dep.as_str());

            let file = match file {
                Some(f) => f,
                None => {
                    continue
                }
            };

            let s = index.get(self.namespace.as_str());
            match s {
                Some(me) => {
                    let mut write = file.write().unwrap();
                    write.add_dependant(me.clone());
                },
                None => ()
            }


            refs.push(file.clone());
        }

        self.depends_on = refs;
    }


    pub fn add_dependant(&mut self, dependant: Arc<RwLock<FileModel>>) {
        self.depended_by.push(dependant.clone());
    }



    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn namespace(&self) -> String {
        self.namespace.clone()
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn functions(&self) -> &Option<Vec<FunctionModel>> {
        &self.functions
    }
}