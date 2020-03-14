use crate::lexer::Token;


pub struct FunctionDef {
    name: String,
    privacy: Option<String>
}


impl FunctionDef {
    pub fn new() -> Self {
        FunctionDef {
            name: String::with_capacity(20),
            privacy: None
        }
    }


    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }


    pub fn set_privacy(&mut self, privacy: Option<String>) {
        self.privacy = privacy;
    }


    pub fn name(&self) -> &String {
        &self.name
    }


    pub fn privacy(&self) -> Option<&String> {
        self.privacy.as_ref()
    }


    pub fn parse(&mut self, token: Token) {
        match token {
            Token::FunctionName(n) => self.set_name(n),
            Token::FunctionPrivacy(p) => self.set_privacy(p),
            _ => ()
        }
    }
}