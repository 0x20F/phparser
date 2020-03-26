mod token;

use std::io::{BufRead, BufReader};
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
        Regex::new("(?P<privacy>[a-zA-Z]+)?(?: )?(?:static )?function (?P<name>[a-zA-Z0-9_]+)\\(")
        .unwrap();


    static ref CLASS: Regex =
        Regex::new("^(abstract |final )?class (?P<name>[a-zA-Z_]+)") // Doesn't handle extensions yet
        .unwrap();
}




pub struct Lexer {}


impl Lexer {
    pub fn tokenize(stream: &mut BufReader<File>) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut stack: Vec<bool> = vec![];

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
                tokens.push(Token::Namespace(namespace["path"].to_string()));

                n = true;
            }


            // Parse dependency if they haven't been parsed already
            if !f && !c && !u && IMPORT.is_match(&line) {
                let import = IMPORT.captures(&line).unwrap();
                tokens.push(Token::Import(import["path"].to_string()));
            }


            // Check if this is a class declaration only if not already in a function or class
            if !f && !c && CLASS.is_match(&line) {
                let class = Self::tokenize_class_definition(&line);
                tokens.extend(class);

                c = true;
            }


            // Check if this is a function declaration only if not already in a function
            if !f && FUNCTION.is_match(&line) {
                let function = Self::tokenize_function_definition(&line);
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
                        tokens.push(Token::ClassEnd);
                        c = false;
                    }

                    if f {
                        tokens.push(Token::FunctionEnd);
                        f = false;
                    }
                }

                if stack.len() == 1 && f {
                    tokens.push(Token::FunctionEnd);
                    f = false;
                }
            }
        }


        tokens
    }


    fn tokenize_class_definition(def: &str) -> Vec<Token> {
        let def = CLASS.captures(def).unwrap();

        vec![
            Token::ClassStart,
            Token::ClassName(def["name"].to_owned())
        ]
    }


    fn tokenize_function_definition(def: &str) -> Vec<Token> {
        let def = FUNCTION.captures(def).unwrap();

        let mut tokens = vec![
            Token::FunctionStart,
            Token::FunctionName(def["name"].to_owned())
        ];

        tokens.push(match def.name("privacy") {
            Some(_) => Token::FunctionPrivacy(Some(def["privacy"].to_owned())),
            None    => Token::FunctionPrivacy(None)
        });

        tokens
    }
}






pub struct Lexemes<'a> {
    code: &'a str,
    special: &'a [char]
}

impl<'a> Lexemes<'a> {
    pub fn from(code: &'a str) -> Self {
        Self {
            code,
            special: &['(', ')', '{', '}']
        }
    }

    fn update(&mut self, margin: usize) {
        self.code = &self.code[margin..].trim();
    }

    fn is_special(&self, c: &char) -> bool {
        self.special.contains(c)
    }
}

impl<'a> Iterator for Lexemes<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token = None;

        if self.code.is_empty() {
            return None;
        }

        let first = self.code.chars().next().unwrap_or_default();

        // If the character is special,
        // return only the first occurence of that
        // character
        if self.is_special(&first) {
            token = Some(&self.code[..1]);
            self.update(1);

            return token;
        }

        // If the character wasn't special,
        // get everything until a space or until
        // we hit another special character
        let rest = self.code.char_indices()
            .take_while(|(_, c)| !c.is_whitespace() && !self.is_special(&c))
            .last()
            .map(|(i, c)| i + c.len_utf8())
            .unwrap_or_default();


        token = Some(&self.code[..rest]);
        self.update(rest);

        token
    }
}








#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn lex() {
        let code = "class A { function b() {} }";
        let mut tokens = Lexemes::from(code);

        for token in tokens {
            println!("{:?}", token);
        }
    }
}