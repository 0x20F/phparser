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


    pub fn goto(&mut self, line: u64) {
        if line > self.position {
            self.buffer.seek(SeekFrom::Current(line as i64)).unwrap();
        } else {
            self.buffer.seek(SeekFrom::Start(line)).unwrap();
        }

        self.position = line;
    }


    pub fn next_line(&mut self) -> Option<String> {
        let mut buf: Vec<u8> = vec![];

        // Get everything 'til the end of the line
        self.buffer.by_ref().read_until(b'\n', &mut buf).unwrap();

        String::from_utf8(buf).ok()
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

        let name = FileDef::parse_name(&path);
        let mut stream = FileDef::open_file(&path);

        let tokens = Lexer::tokenize(&mut stream);

        for token in tokens {
            match token {
                Token::Namespace(pos) => {
                    namespace = FileDef::parse_namespace(pos, &mut stream);
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


    fn parse_namespace(line: u64, stream: &mut FileStream) -> Option<String> {
        stream.goto(line);
        stream.next_line() // For now, gonna need to actually get the namespace from that line
    }


    fn open_file(path: &PathBuf) -> FileStream {
        FileStream::new(path)
    }
}