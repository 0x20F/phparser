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

        let name = FileDef::parse_name(&path);
        let mut stream = FileStream::new(&path);

        let tokens = Lexer::tokenize(&mut stream);

        for token in tokens {
            match token {
                Token::Namespace(pos, n) => namespace = Some(n),
                Token::Import(pos, i) => dependencies.push(i),

                Token::ClassStart(pos) => println!("Class starts on {}", pos),
                Token::ClassEnd(pos) => println!("Class ends on {}", pos),
                _ => break
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

        pieces.last().unwrap().to_string()
    }


    fn extract_path(declaration: String) -> String {
        let keywords: Vec<&str> = declaration.split(' ').collect();

        let path = keywords.last().unwrap();
        let path = path.trim().trim_end_matches(';');

        path.to_string()
    }
}