use super::definitions::{FileStream};
use std::io::{Read, BufRead};



pub struct Token {
    pub token: TokenType,
    pub line: u64,
    pub symbol: String
}

impl Token {
    pub fn new(token: TokenType, line: u64, symbol: String) -> Token {
        Token {
            token,
            line,
            symbol
        }
    }
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
        let mut stack: Vec<u32> = vec![];

        let mut line_number = stream.current_line();

        /*
            Flags:

            n => namespace already done
        */

        let (mut n) = (false);

        
        for line in stream.buffer.by_ref().lines() {
            let line = match line {
                Ok(line) => line,
                _ => break
            };

            // Parse namespace if it hasn't been parsed already
            if !n && line.starts_with("namespace") {
                let token = Lexer::parse_namespace(line, line_number);
                tokens.push(token);
                n = true;
            }



            line_number = line_number + 1;
        }


        tokens
    }


    pub fn parse_namespace(line_text: String, line_number: u64) -> Token {
        let namespace = line_text
            .trim()
            .split_whitespace()
            .last()
            .unwrap()
            .trim_end_matches(';');

        let token = Token::new(
            TokenType::Namespace,
            line_number,
            namespace.to_string()
        );

        token
    }
}