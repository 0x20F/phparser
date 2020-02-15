use super::definitions::{FileStream};
use std::io::{Read, BufRead};
use core::borrow::{BorrowMut};


pub enum Tokens {
    NotFound(),

    Namespace(u64)
}



pub struct Lexer {}


impl Lexer {
    pub fn tokenize(stream: &mut FileStream) -> Vec<Tokens> {
        let mut tokens: Vec<Tokens> = vec![];

        tokens.push(Lexer::find_namespace(stream.borrow_mut()));

        tokens
    }


    pub fn find_namespace(stream: &mut FileStream) -> Tokens {

        let mut token = Tokens::NotFound();
        let mut line_number = stream.current_line();


        for line in stream.buffer.by_ref().lines() {
            let line = match line {
                Ok(line) => line,
                Err(_e) => break
            };

            if line.contains("namespace") {
                token = Tokens::Namespace(line_number);
                break;
            }

            line_number = line_number + 1;
        }

        stream.set_lines(line_number);
        token
    }
}