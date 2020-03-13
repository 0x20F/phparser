use crate::lexer::Token;


pub struct FunctionDef {
    name: String,
    privacy: Option<String>
}


impl FunctionDef {
    pub fn new<I>(tokens: &mut I) -> Self
        where I: Iterator<Item = Token>
    {
        let mut token = tokens.next().unwrap();
        let (mut name, mut privacy) = (String::with_capacity(20), None);

        loop {
            match token {
                Token::FunctionName(_, n) => name = n,
                Token::FunctionPrivacy(_, p) => privacy = p,
                Token::FunctionEnd(_) => break,
                _ => ()
            }

            if let Some(t) = tokens.next() {
                token = t;
            } else {
                break;
            }
        }

        FunctionDef {
            name,
            privacy
        }
    }
}