use super::definitions::{FileStream};
use std::io::{Read, BufRead};



pub struct Token {
    token: TokenType,
    line: u64,
    symbol: String
}


pub enum TokenType {
    NotFound,

    Namespace,
    Use,

    ClassStart,
    ClassEnd
}



pub struct Lexer {}


impl Lexer {
    pub fn tokenize(stream: &mut FileStream) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let mut line_number = stream.current_line();
        
        for line in stream.buffer.by_ref().lines() {
            let line = match line {
                Ok(line) => line,
                _ => break
            };

            

            line_number = line_number + 1;
        }


        tokens
    }
}