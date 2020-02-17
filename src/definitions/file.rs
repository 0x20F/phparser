use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};
use std::path::{PathBuf};
use crate::lexer::{Lexer, Token};




pub struct FileStream {
    pub buffer: BufReader<File>,
    current_line: u64
}


impl FileStream {
    pub fn new(path: &PathBuf) -> FileStream {
        // Make sure you fail gracefully here
        let file = File::open(path).unwrap();

        let buffer = BufReader::new(file);

        FileStream {
            buffer,
            current_line: 1
        }
    }


    pub fn rewind(&mut self) {
        // Seek back to the start so other things can use this same buffer
        // without reopening the file every time
        self.buffer.seek(SeekFrom::Start(0)).unwrap();
        self.current_line = 1;
    }


    pub fn set_lines(&mut self, lines: u64) {
        self.current_line = self.current_line + lines;
    }


    pub fn current_line(&self) -> u64 {
        self.current_line
    }
}





pub struct FileDef {
    pub path: PathBuf,
    pub name: String,
}


impl FileDef {
    pub fn new(path: PathBuf) -> FileDef {

        let name = FileDef::parse_name(&path);
        let mut stream = FileDef::open_file(&path);

        let tokens = Lexer::tokenize(&mut stream);

        for token in tokens {
            match token {
                Token::Namespace(line) => println!("Namespace is on line {}", line),
                _ => break
            }
        }


        FileDef {
            path,
            name
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


    fn open_file(path: &PathBuf) -> FileStream {
        FileStream::new(path)
    }
}