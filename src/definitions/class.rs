use crate::lexer::Token;
use crate::definitions::FunctionDef;


pub struct ClassDef {
    name: String,
    methods: Vec<FunctionDef>
}


impl ClassDef {
    pub fn new<I>(mut tokens: &mut I) -> Self
        where I: Iterator<Item = Token>
    {
        let mut token = tokens.next().unwrap();

        let mut methods = vec![];
        let mut name = String::with_capacity(20);

        loop {
            match token {
                Token::ClassName(n) => name = n,

                Token::FunctionStart => methods.push(FunctionDef::new(&mut tokens)),

                Token::ClassEnd => break,
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
            methods
        }
    }


    pub fn name(&self) -> &String {
        &self.name
    }


    pub fn methods(&self) -> Option<&Vec<FunctionDef>> {
        if self.methods.is_empty() {
            return None;
        }

        Some(&self.methods)
    }
}









#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_with_methods() {
        let name = String::from("TestClass");
        let fn_name = String::from("test_function");

        let tokens = vec![
            Token::ClassStart,
            Token::ClassName(name.clone()),

            Token::FunctionStart,
            Token::FunctionName(fn_name.clone()),
            Token::FunctionEnd,

            Token::FunctionStart,
            Token::FunctionName(fn_name.clone()),
            Token::FunctionEnd,

            Token::ClassEnd
        ];

        let class = ClassDef::new(&mut tokens.into_iter());

        // Name is the same
        assert_eq!(*class.name(), name);

        // Methods exist
        assert_eq!(class.methods().unwrap().len(), 2);
    }


    #[test]
    fn create_without_methods() {
        let name = String::from("TestClass");

        let tokens = vec![
            Token::ClassStart,
            Token::ClassName(name.clone()),

            Token::ClassEnd
        ];

        let class = ClassDef::new(&mut tokens.into_iter());

        // There should be no methods
        assert!(class.methods().is_none());
    }
}