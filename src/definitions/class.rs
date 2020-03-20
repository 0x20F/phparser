use crate::lexer::Token;
use crate::definitions::{ FunctionDef };


#[derive(Default)]
pub struct ClassDef {
    name: String,
    methods: Vec<FunctionDef>
}


impl ClassDef {
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
        self.methods.push(FunctionDef::default());
    }


    fn last_method(&mut self) -> Option<&mut FunctionDef> {
        self.methods.last_mut()
    }
}