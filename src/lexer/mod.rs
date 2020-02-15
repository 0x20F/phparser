use super::definitions::{FileDef, FileStream};
use std::path::PathBuf;
use std::io::{BufReader, Read, BufRead, Seek};
use std::fs::File;
use core::borrow::BorrowMut;


pub enum Tokens {
    NotFound(),

    Namespace(i64)
}



pub struct Lexer {}


impl Lexer {
    pub fn tokenize(stream: &mut FileStream) -> Vec<Tokens> {
        let mut tokens: Vec<Tokens> = vec![];

        tokens.push(Lexer::find_namespace(stream.borrow_mut()));

        tokens
    }


    pub fn find_namespace(stream: &mut FileStream) -> Tokens {
        let mut line_count = 0;

        for line in stream.buffer.by_ref().lines() {
            if line.is_err() {
                // If one line in a file fails, all others probably will too
                // so you should just return here to save time
                // Don't forget to handle it properly on the other end
                continue;
            }

            let line = line.unwrap();

            if line.contains("namespace") {
                return Tokens::Namespace(line_count as i64);
            }

            line_count = line_count + 1;
        }

        Tokens::NotFound()
    }
}