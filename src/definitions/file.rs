use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};
use std::path::{PathBuf};
use crate::lexer::Lexer;




pub struct FileStream {
    buffer: BufReader<File>
}


impl FileStream {
    pub fn new(path: &PathBuf) -> FileStream {
        // Make sure you fail gracefully here
        let file = File::open(path).unwrap();

        let mut buffer = BufReader::new(file);

        FileStream {
            buffer
        }
    }


    pub fn rewind(&mut self) {
        // Seek back to the start so other things can use this same buffer
        // without reopening the file every time
        self.buffer.seek(SeekFrom::Start(0)).unwrap();
    }


    pub fn goto(&mut self, position: u64) {
        self.buffer.seek(SeekFrom::Start(position)).unwrap();
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

        let tokens = Lexer::tokenize(&stream);


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