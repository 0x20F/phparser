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
        let mut stream = Self::open_file(&path);

        // Turn it into an iterator to allow more control
        let tokens = Lexer::tokenize(&mut stream);

        let mut in_class = false;

        for token in tokens {
            match token {
                Token::ClassStart => {
                    def.add_class();
                    in_class = true;
                },
                Token::ClassEnd => in_class = false,

                // If in class, pass tokens on to class definition
                _ if in_class => {
                    if let Some(class) = def.classes.last_mut() {
                        class.take(token);
                    }
                }

                // If still in this file def, pass tokens to self since tokens
                // probably have to do with this file/this file's children
                _ => def.take(token)
            }
        }


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


    fn add_class(&mut self) {
        self.classes.push(ClassDef::default());
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


    fn open_file(path: &PathBuf) -> BufReader<File> {
        // Make sure you fail gracefully here
        let file = File::open(path).unwrap();

        BufReader::new(file)
    }
}



impl ExtractTokens for FileDef {
    fn take(&mut self, token: Token) {
        match token {
            Token::Namespace(n) => self.namespace = Some(n),
            Token::Import(i) => self.dependencies.push(i),

            Token::FunctionStart => self.add_function(),

            _ => {
                if let Some(function) = self.last_function() {
                    function.take(token);
                }
            }
        }
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