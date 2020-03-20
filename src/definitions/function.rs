use crate::lexer::Token;


#[derive(Default)]
pub struct FunctionDef {
    name: String,
    privacy: Option<String>
}


impl FunctionDef {
    pub fn name(&self) -> &String {
        &self.name
    }


    pub fn privacy(&self) -> Option<&String> {
        self.privacy.as_ref()
    }
}