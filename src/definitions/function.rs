use crate::lexer::Token;
use crate::definitions::ExtractTokens;


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


    pub fn name(&self) -> &String {
        &self.name
    }


    pub fn privacy(&self) -> Option<&String> {
        self.privacy.as_ref()
    }
}


impl ExtractTokens for FunctionDef {
    fn take(&mut self, token: Token) {
        match token {
            Token::FunctionName(n) => self.name = n,
            Token::FunctionPrivacy(p) => self.privacy = p,
            _ => ()
        }
    }
}