mod token;

use super::definitions::{FileStream};
use std::io::{Read, BufRead, BufReader};
use regex::Regex;

pub use token::Token;
use std::fs::File;


lazy_static! {
    static ref NAMESPACE: Regex =
        Regex::new("namespace (?P<path>.+);")
        .unwrap();


    static ref IMPORT: Regex =
        Regex::new("use (?P<path>.+);")
        .unwrap();


    static ref FUNCTION: Regex =
        Regex::new("(?P<privacy>[a-zA-Z]+)?(?: )?(?:static )?function (?P<name>[a-zA-Z0-9]+)\\(")
        .unwrap();


    static ref CLASS: Regex =
        Regex::new("^(abstract |final )?class (?P<name>[a-zA-Z]+)") // Doesn't handle extensions yet
        .unwrap();
}




pub struct Lexer {}


impl Lexer {
    pub fn tokenize(stream: &mut BufReader<File>) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut stack: Vec<bool> = vec![];

        let mut pos = 0;

        /*
            Flags:

            n => namespace already done
            c => currently inside a class
            f => currently inside a function
            u => use/import statements have already been parsed
        */
        let (mut n, mut c, mut f, mut u) = (false, false, false, false);

        
        for line in stream.lines() {
            let line = match line {
                Ok(line) => line,
                _ => break
            };

            // Parse namespace if it hasn't been parsed already
            if !n && NAMESPACE.is_match(&line) {
                let namespace = NAMESPACE.captures(&line).unwrap();
                tokens.push(Token::Namespace(pos, namespace["path"].to_string()));

                n = true;
            }


            // Parse dependency if they haven't been parsed already
            if !f && !c && !u && IMPORT.is_match(&line) {
                let import = IMPORT.captures(&line).unwrap();
                tokens.push(Token::Import(pos, import["path"].to_string()));
            }


            // Check if this is a class declaration only if not already in a function or class
            if !f && !c && CLASS.is_match(&line) {
                let class = Self::tokenize_class_definition(pos, &line);
                tokens.extend(class);

                c = true;
            }


            // Check if this is a function declaration only if not already in a function
            if !f && FUNCTION.is_match(&line) {
                let function = Self::tokenize_function_definition(pos, &line);
                tokens.extend(function);

                f = true;
            }


            // Don't think this'll work if everything is on the same line
            if line.contains('{') { stack.push(true); }
            if line.contains('}') {
                stack.pop();

                // If we're already in a class or function we shouldn't
                // expect any import or namespace statements anymore
                if c || f {
                    u = true;
                    n = true;
                }

                if stack.is_empty() {
                    if c {
                        tokens.push(Token::ClassEnd(pos));
                        c = false;
                    }

                    if f {
                        tokens.push(Token::FunctionEnd(pos));
                        f = false;
                    }
                }

                if stack.len() == 1 && f {
                    tokens.push(Token::FunctionEnd(pos));
                    f = false;
                }
            }

            // +1 to account for the newline character that the buffer removes
            pos = (pos + line.len() as u64) + 1;
        }


        tokens
    }


    fn tokenize_class_definition(pos: u64, def: &str) -> Vec<Token> {
        let def = CLASS.captures(def).unwrap();

        vec![
            Token::ClassStart(pos),
            Token::ClassName(pos, def["name"].to_owned())
        ]
    }


    fn tokenize_function_definition(pos: u64, def: &str) -> Vec<Token> {
        let def = FUNCTION.captures(def).unwrap();

        let mut tokens = vec![
            Token::FunctionStart(pos),
            Token::FunctionName(pos, def["name"].to_owned())
        ];

        tokens.push(match def.name("privacy") {
            Some(_) => Token::FunctionPrivacy(pos, Some(def["privacy"].to_owned())),
            None    => Token::FunctionPrivacy(pos, None)
        });

        tokens
    }
}