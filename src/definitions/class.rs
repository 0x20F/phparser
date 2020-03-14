use crate::lexer::Token;
use crate::definitions::FunctionDef;


pub struct ClassDef {
    name: String,
    methods: Vec<FunctionDef>
}


impl ClassDef {
    pub fn new() -> Self {
        ClassDef {
            name: String::with_capacity(20),
            methods: vec![]
        }
    }


    pub fn set_name(&mut self, name: String) {
        self.name = name;
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


    fn new_method(&mut self) {
        self.methods.push(FunctionDef::new());
    }


    fn last_method(&mut self) -> Option<&mut FunctionDef> {
        self.methods.last_mut()
    }


    pub fn parse(&mut self, token: Token) {
        match token {
            Token::ClassName(n) => self.set_name(n),

            Token::FunctionStart => self.new_method(),
            Token::FunctionEnd => (),
            _ => {
                match self.last_method() {
                    Some(m) => m.parse(token),
                    _ => ()
                }
            }
        }
    }
}









/*#[cfg(test)]
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

        let class = ClassDef::new();

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

        let class = ClassDef::new();

        // There should be no methods
        assert!(class.methods().is_none());
    }
}*/