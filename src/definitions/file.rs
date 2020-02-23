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
        // Seek back to the start so other things can use this same buffer
        // without reopening the file every time
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
    pub namespace: Option<String>
}


impl FileDef {
    pub fn new(path: PathBuf) -> FileDef {

        let mut namespace = None;
        let mut dependencies = vec![];

        let name = FileDef::parse_name(&path);
        let mut stream = FileStream::new(&path);

        let tokens = Lexer::tokenize(&mut stream);

        for token in tokens {
            match token {
                Token::Namespace(pos) => {
                    namespace = Some(FileDef::parse_namespace(pos, &mut stream));
                },

                Token::Use(pos) => {
                    dependencies.push(FileDef::parse_dependency(pos, &mut stream));
                },

                Token::ClassStart(pos) => println!("Class starts on {}", pos),
                Token::ClassEnd(pos) => println!("Class ends on {}", pos),
                _ => break
            }
        }


        FileDef {
            path,
            name,
            namespace
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


    fn parse_namespace(line: u64, stream: &mut FileStream) -> String {
        stream.jump_to(line);

        let declaration = stream.next_line();

        FileDef::extract_path(declaration)
    }


    fn parse_dependency(line: u64, stream: &mut FileStream) -> String {
        stream.jump_to(line);

        let declaration = stream.next_line();

        FileDef::extract_path(declaration)
    }


    fn extract_path(declaration: String) -> String {
        let keywords: Vec<&str> = declaration.split(' ').collect();

        let path = keywords.last().unwrap();
        let path = path.trim().trim_end_matches(';');

        path.to_string()
    }
}