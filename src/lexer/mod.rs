use super::definitions::{FileStream};
use std::io::{Read, BufRead};
use regex::Regex;





pub enum Token {
    Namespace(u64),
    Use(u64),

    Class(u64, u64),

    Method(u64, u64),

    Function(u64, u64)
}




pub struct Lexer {}


impl Lexer {
    pub fn tokenize(stream: &mut FileStream) -> Vec<Token> {
        lazy_static! {
            static ref NAMESPACE: Regex =
                Regex::new("(?:namespace )(.*)(?:;)")
                .unwrap();

            static ref FUNCTION: Regex =
                Regex::new("(?:public|private|protected|^)( *)?(?:static )?function (?:[A-Za-z0-9]+)\\(")
                .unwrap();
        }

        let mut tokens: Vec<Token> = vec![];
        let mut stack: Vec<bool> = vec![];

        let mut position = 0;

        /*
            Flags:

            n => namespace already done
            c => currently inside a class
            f => currently inside a function
        */
        let (mut n, mut c, mut f) = (false, false, false);
        /*
            cs => class start position
            fs => function/method start position

            ce => class end position
            fe => function/method end position
        */
        let (mut cs, mut fs) = (0, 0);
        let (mut ce, mut fe);

        
        for line in stream.buffer.by_ref().lines() {
            let line = match line {
                Ok(line) => line,
                _ => break
            };

            // Parse namespace if it hasn't been parsed already
            if !n && NAMESPACE.is_match(&line) {
                tokens.push(Token::Namespace(position));
                n = true;
            }


            // Check if this is a class declaration only if not already in a function or class
            if !f && !c && line.starts_with("class") {
                cs = position;
                c = true;
            }


            // Check if this is a function declaration only if not already in a function
            if !f && FUNCTION.is_match(&line) {
                fs = position;
                f = true;
            }


            // Don't think this'll work if everything is on the same line
            if line.contains('{') { stack.push(true); }
            if line.contains('}') {
                stack.pop();

                if stack.len() == 0 {
                    if c {
                        ce = position;
                        tokens.push(Token::Class(cs, ce));
                        c = false;
                    }

                    if f {
                        fe = position;
                        tokens.push(Token::Function(fs, fe));
                        f = false;
                    }
                }

                if stack.len() == 1 {
                    if f {
                        fe = position;

                        if c {
                            tokens.push(Token::Method(fs, fe));
                        } else {
                            tokens.push(Token::Function(fs, fe));
                        }

                        f = false;
                    }
                }
            }

            // +1 to account for the newline character that the buffer removes
            position = (position + line.len() as u64) + 1;
        }


        tokens
    }
}