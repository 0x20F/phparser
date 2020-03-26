#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod definitions;
pub mod lexer;

use walkdir::{WalkDir};
use definitions::FileDef;
use std::path::{Path, PathBuf};
use std::io::BufReader;
use std::fs::File;
use crate::lexer::{Lexer, Token};


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


    pub fn parse_dir(&self, dir: &str) -> Vec<FileDef> {
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

        files
    }


    pub fn parse_file(&self, file_path: PathBuf) -> FileDef {
        let mut file = Self::open_file(&file_path);
        let tokens = Lexer::tokenize(&mut file);

        let mut file_def = FileDef::new(file_path);

        for token in tokens {
            match token {
                Token::Import(i) => file_def.add_dependency(&i),
                Token::Namespace(n) => file_def.set_namespace(&n),
                _ => println!("{:?}", token)
            }
        }

        file_def
    }


    fn open_file(file_path: &PathBuf) -> BufReader<File> {
        let file = File::open(file_path).unwrap();

        BufReader::new(file)
    }
}

