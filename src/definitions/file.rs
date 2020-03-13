use std::fs::File;
use std::io::BufReader;
use std::path::{PathBuf};
use crate::lexer::{Lexer, Token};
use crate::definitions::ClassDef;





pub struct FileDef {
    path: PathBuf,
    name: String,
    namespace: Option<String>,
    dependencies: Vec<String>,
    classes: Vec<ClassDef>
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
        let mut tokens = Lexer::tokenize(&mut stream).into_iter();

        let mut classes = vec![];

        if let Some(first) = tokens.next() {
            let mut token = first;

            loop {
                match token {
                    Token::Namespace(_, n) => namespace = Some(n),
                    Token::Import(_, i) => dependencies.push(i),

                    Token::ClassStart(_) => classes.push(ClassDef::new(&mut tokens)),
                    _ => break
                }

                if let Some(t) = tokens.next() {
                    token = t;
                } else {
                    break;
                }
            }
        }


        FileDef {
            path,
            name,
            namespace,
            dependencies,
            classes
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