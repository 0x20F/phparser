use super::definitions::{FileDef, FileStream};
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;


pub enum Tokens {
    Namespace(i32)
}



pub struct Lexer {}


impl Lexer {
    pub fn tokenize(stream: &FileStream) -> Vec<Tokens> {

    }
}