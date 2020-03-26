pub mod lexer;

use walkdir::{WalkDir};
use std::path::{Path, PathBuf};
use std::io::BufReader;
use std::fs::File;


#[derive(Default)]
pub struct Parser {}


impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }


    pub fn parse(&self, dirs: &[&str]) {
        // Start parsing after creating the parser

        // Walk through all items in each directory
        for dir in dirs {
            self.parse_dir(dir);
        }
    }


    pub fn parse_dir(&self, dir: &str) {
        // Find all files and go down the parse chain
        let walker = WalkDir::new(dir).into_iter();

        // All files in the given directory
        let mut files = vec![];

        // Filter away the paths that aren't accessible
        for entry in walker.filter_map(|e| e.ok()) {
            if entry.path().is_dir() {
                continue;
            }

            files.push(self.parse_file(entry.path().to_owned()));
        }
    }


    pub fn parse_file(&self, file_path: PathBuf) {
        unimplemented!();
    }


    fn open_file(file_path: &PathBuf) -> BufReader<File> {
        let file = File::open(file_path).unwrap();

        BufReader::new(file)
    }
}

