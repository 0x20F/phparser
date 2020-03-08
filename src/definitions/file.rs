use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};
use std::path::{PathBuf};
use crate::lexer::{Lexer, Token};




pub struct FileStream {
    pub buffer: BufReader<File>,
    position: u64
}


impl FileStream {
    pub fn new(path: &PathBuf) -> FileStream {
        // Make sure you fail gracefully here
        let file = File::open(path).unwrap();

        let buffer = BufReader::new(file);

        FileStream {
            buffer,
            position: 0
        }
    }


    pub fn rewind(&mut self) {
        self.buffer.seek(SeekFrom::Start(0)).unwrap();
        self.position = 0;
    }


    pub fn jump_to(&mut self, line: u64) {
        self.buffer.seek(SeekFrom::Start(line)).unwrap();

        self.position = line;
    }


    pub fn next_line(&mut self) -> String {
        let mut buf: String = String::new();

        self.buffer.read_line(&mut buf).unwrap();

        buf
    }
}





pub struct FileDef {
    pub path: PathBuf,
    pub name: String,
    pub namespace: Option<String>,
    pub dependencies: Vec<String>
}


impl FileDef {
    pub fn new(path: PathBuf) -> FileDef {

        let mut namespace = None;
        // Dependencies should be references to the files that contain the classes?
        // or maybe just have a function find_dependencies() that returns a list
        // of file object references?
        let mut dependencies = vec![];

        let name = Self::parse_name(&path);
        let mut stream = FileStream::new(&path);

        let mut tokens = Lexer::tokenize(&mut stream).into_iter();

        if let Some(first) = tokens.next() {
            let mut token = first;

            loop {
                match token {
                    Token::Namespace(_, n) => namespace = Some(n),
                    Token::Import(_, i) => dependencies.push(i),

                    Token::ClassStart(_) => Self::build_class(&mut tokens),
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
            dependencies
        }
    }


    fn parse_name(path: &PathBuf) -> String {
        let pieces: Vec<&str> = path
            .to_str()
            .unwrap()
            .split(&['/', '\\'][..])
            .collect();

        (*pieces.last().unwrap()).to_string()
    }


    fn build_class<I>(tokens: &mut I)
        where I: Iterator<Item = Token>
    {
        if let Some(first) = tokens.next() {
            let mut token = first;

            loop {
                match token {
                    Token::ClassEnd(_) => println!("Class end directly"),
                    _ => println!("There are other things")
                }

                if let Some(t) = tokens.next() {
                    token = t;
                } else {
                    break;
                }
            }
        }
    }
}