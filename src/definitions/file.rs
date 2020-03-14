use std::fs::File;
use std::io::BufReader;
use std::path::{PathBuf};
use crate::lexer::{Lexer, Token};
use crate::definitions::{ClassDef, FunctionDef};





pub struct FileDef {
    path: PathBuf,
    name: String,
    namespace: Option<String>,
    dependencies: Vec<String>,
    classes: Vec<ClassDef>,
    functions: Vec<FunctionDef>
}


impl FileDef {
    pub fn new(path: PathBuf) -> FileDef {

        let mut namespace = None;
        // Dependencies should be references to the files that contain the classes?
        // or maybe just have a function find_dependencies() that returns a list
        // of file object references?
        let mut dependencies = vec![];

        let name = Self::parse_name(&path);
        let mut stream = Self::open_file(&path);

        // Turn it into an iterator to allow more control
        let mut tokens = Lexer::tokenize(&mut stream);

        let mut classes = vec![];
        let functions = vec![];

        for token in tokens {
            match token {
                Token::Namespace(n) => namespace = Some(n),
                Token::Import(i) => dependencies.push(i),

                Token::ClassStart => classes.push(ClassDef::new()),
                Token::ClassEnd => (),

                _ => classes.last_mut().unwrap().parse(token)
            }
        }


        FileDef {
            path,
            name,
            namespace,
            dependencies,
            classes,
            functions
        }
    }


    pub fn path(&self) -> &PathBuf {
        &self.path
    }


    pub fn name(&self) -> &String {
        &self.name
    }


    pub fn namespace(&self) -> Option<&String> {
        self.namespace.as_ref()
    }


    pub fn dependencies(&self) -> Option<&Vec<String>> {
        if self.dependencies.is_empty() {
            return None;
        }

        Some(&self.dependencies)
    }


    pub fn classes(&self) -> Option<&Vec<ClassDef>> {
        if self.classes.is_empty() {
            return None;
        }

        Some(&self.classes)
    }


    pub fn add_class(&mut self) {
        self.classes.push(ClassDef::new());
    }


    pub fn last_class(&mut self) -> Option<&mut ClassDef> {
        self.classes.last_mut()
    }


    pub fn add_function(&mut self) {
        self.functions.push(FunctionDef::new());
    }


    pub fn last_function(&mut self) -> Option<&mut FunctionDef> {
        self.functions.last_mut()
    }


    fn parse_name(path: &PathBuf) -> String {
        let pieces: Vec<&str> = path
            .to_str()
            .unwrap()
            .split(&['/', '\\'][..])
            .collect();

        (*pieces.last().unwrap()).to_string()
    }


    fn open_file(path: &PathBuf) -> BufReader<File> {
        // Make sure you fail gracefully here
        let file = File::open(path).unwrap();

        BufReader::new(file)
    }
}