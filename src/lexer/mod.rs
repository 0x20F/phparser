use super::definitions::{FileStream};
use std::io::{Read, BufRead};
use core::borrow::{BorrowMut};


pub enum Tokens {
    NotFound(),

    Namespace(u64),
    Use(u64),

    ClassStart(u64),
    ClassEnd(u64)
}



pub struct Lexer {}


impl Lexer {
    pub fn tokenize(stream: &mut FileStream) -> Vec<Tokens> {
        let mut tokens: Vec<Tokens> = vec![];

        // File header, assume that namespace and use statements
        // are declared at the top
        tokens.push(Lexer::find_namespace(stream.borrow_mut()));
        tokens.extend(Lexer::find_dependencies(stream.borrow_mut()));

        // Rewind


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


    pub fn find_dependencies(stream: &mut FileStream) -> Vec<Tokens> {
        let mut tokens = vec![];
        let mut line_number = stream.current_line();

        for line in stream.buffer.by_ref().lines() {
            let line = match line {
                Ok(line) => line,
                Err(_e) => break
            };

            // Gonna need to account for PHP7+ use statements aswell
            // https://www.php.net/manual/en/language.namespaces.importing.php
            if line.starts_with("use") {
                tokens.push(Tokens::Use(line_number));
            }

            // Don't continue if class declaration is reached
            /*if line.starts_with("class") {
                break;
            }*/

            line_number = line_number + 1;
        }

        stream.set_lines(line_number);
        tokens
    }


    pub fn find_classes(stream: &mut FileStream) -> Vec<Tokens> {
        let mut tokens = vec![];
        let mut stack: Vec<u16> = vec![];
        let mut line_number = stream.current_line();

        for line in stream.buffer.by_ref().lines() {
            let line = match line {
                Ok(line) => line,
                Err(_e) => break
            };

            // Class declaration found
            if line.starts_with("class") {
                tokens.push(Tokens::ClassStart(line_number));
            }

            if line.contains("{") {
                stack.push(1);
            }

            if line.contains("}") {
                stack.pop();

                // Reached the end of the class
                if stack.len() == 0 {
                    tokens.push(Tokens::ClassEnd(line_number));
                }
            }

            line_number = line_number + 1;
        }

        tokens
    }
}