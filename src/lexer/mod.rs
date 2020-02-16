use super::definitions::{FileStream};
use std::io::{Read, BufRead};
use regex::Regex;





pub enum Token {
    Namespace(u64),
    Use(u64),

    ClassStart(u64),
    ClassEnd(u64),

    MethodStart(u64),
    MethodEnd(u64),

    FunctionStart(u64),
    FunctionEnd(u64)
}




pub struct Lexer {}


impl Lexer {
    pub fn tokenize(stream: &mut FileStream) -> Vec<Token> {
        lazy_static! {
            static ref namespace_declaration: Regex =
                Regex::new("(?:namespace )(.*)(?:;)")
                .unwrap();

            static ref function_declaration: Regex =
                Regex::new("(?:public|private|protected|^)( *)?(?:static )?function (?:[A-Za-z0-9]+)\\(")
                .unwrap();
        }

        let mut tokens: Vec<Token> = vec![];
        let mut stack: Vec<bool> = vec![];

        let mut line_number = stream.current_line();

        /*
            Flags:

            n => namespace already done
            c => currently inside a class
            f => currently inside a function
        */
        let (mut n, mut c, mut f) = (false, false, false);

        
        for line in stream.buffer.by_ref().lines() {
            let line = match line {
                Ok(line) => line,
                _ => break
            };

            // Parse namespace if it hasn't been parsed already
            if !n && namespace_declaration.is_match(&line) {
                tokens.push(Token::Namespace(line_number));
                n = true;
            }


            // Check if this is a class declaration only if not already in a function or class
            if !f && !c && line.starts_with("class") {
                tokens.push(Token::ClassStart(line_number));
                c = true;
            }


            // Check if this is a function declaration only if not already in a function
            if !f && function_declaration.is_match(&line) {
                if c {
                    tokens.push(Token::MethodStart(line_number));
                } else {
                    tokens.push(Token::FunctionStart(line_number));
                }

                f = true;
            }



            if line.contains('{') { stack.push(true); }
            if line.contains('}') {
                stack.pop();

                if stack.len() == 0 {
                    if c {
                        tokens.push(Token::ClassEnd(line_number));
                        c = false;
                    }

                    if f {
                        tokens.push(Token::FunctionEnd(line_number));
                        f = false;
                    }
                }

                if stack.len() == 1 {
                    if f {
                        tokens.push(Token::FunctionEnd(line_number));
                        f = false;
                    }
                }
            }

            line_number = line_number + 1;
        }


        tokens
    }
}