use std::fs::File;
use std::io::BufReader;
use std::path::{PathBuf};
use crate::lexer::{Lexer, Token};
use crate::definitions::{ClassDef, FunctionDef, ExtractTokens};





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
        let mut def = FileDef::default();

        let name = Self::parse_name(&path);

        def.path = path;
        def.name = name;
        def
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


    pub fn set_namespace(&mut self, namespace: &str) {
        self.namespace = Some(namespace.to_owned())
    }


    pub fn add_dependency(&mut self, dependency: &str) {
        // Take a ClassDef reference or something later on...
        self.dependencies.push(dependency.to_owned())
    }


    fn add_class(&mut self) {
        self.classes.push(ClassDef::default());
    }


    fn last_class(&mut self) -> Option<&mut ClassDef> {
        self.classes.last_mut()
    }


    fn add_function(&mut self) {
        self.functions.push(FunctionDef::default());
    }


    fn last_function(&mut self) -> Option<&mut FunctionDef> {
        self.functions.last_mut()
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


    fn parse_name(path: &PathBuf) -> String {
        let pieces: Vec<&str> = path
            .to_str()
            .unwrap()
            .split(&['/', '\\'][..])
            .collect();

        (*pieces.last().unwrap()).to_string()
    }
}



impl Default for FileDef {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            name: String::from(""),
            namespace: None,
            dependencies: vec![],
            classes: vec![],
            functions: vec![]
        }
    }
}