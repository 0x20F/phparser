use crate::lexer::Token;
use crate::definitions::FunctionDef;


pub struct ClassDef {
    name: String,
    methods: Option<Vec<FunctionDef>>
}


impl ClassDef {
    pub fn new<I>(mut tokens: &mut I) -> Self
        where I: Iterator<Item = Token>
    {
        let mut token = tokens.next().unwrap();

        let mut methods = vec![];
        let mut name = String::with_capacity(20);

        // Loop through tokens until ClassEnd is reached
        // This should all build a ClassDef object
        loop {
            match token {
                Token::ClassName(_, n) => name = n,

                Token::FunctionStart(_) => methods.push(FunctionDef::new(&mut tokens)),

                Token::ClassEnd(_) => break,
                _ => ()
            }

            if let Some(t) = tokens.next() {
                token = t;
            } else {
                break;
            }
        }

        ClassDef {
            name,
            methods: if methods.is_empty() { None } else { Some(methods) }
        }
    }
}