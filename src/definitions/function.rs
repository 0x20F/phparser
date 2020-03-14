use crate::lexer::Token;
use crate::definitions::ExtractTokens;


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


impl ExtractTokens for FunctionDef {
    fn take(&mut self, token: Token) {
        match token {
            Token::FunctionName(n) => self.name = n,
            Token::FunctionPrivacy(p) => self.privacy = p,
            _ => ()
        }
    }
}