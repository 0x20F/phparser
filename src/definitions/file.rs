use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};
use std::path::{Path, PathBuf};
use std::fmt::Display;

pub struct FileDef {
    pub path: PathBuf,
    pub name: String,
}


impl FileDef {
    pub fn new(path: PathBuf) -> FileDef {

        let name = FileDef::parse_name(&path);
        let mut stream = FileDef::open_file(&path);

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


    fn open_file(path: &PathBuf) -> BufReader<File> {
        // Make sure you fail gracefully here!
        let f = File::open(path).unwrap();

        let mut f = BufReader::new(f);

        f
    }


    fn rewind(buffer: &mut BufReader<File>) {
        // Seek back to the start so other things can use this same buffer
        buffer.seek(SeekFrom::Start(0)).unwrap();
    }
}