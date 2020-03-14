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
                Token::FunctionName(n) => name = n,
                Token::FunctionPrivacy(p) => privacy = p,
                Token::FunctionEnd => break,
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


    pub fn name(&self) -> &String {
        &self.name
    }


    pub fn privacy(&self) -> Option<&String> {
        self.privacy.as_ref()
    }
}









#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_method() {
        let name = String::from("test_function");
        let privacy = Some(String::from("protected"));

        let tokens = vec![
            Token::FunctionStart,
            Token::FunctionName(name.clone()),
            Token::FunctionPrivacy(privacy.clone()),
            Token::FunctionEnd
        ];

        let function = FunctionDef::new(&mut tokens.into_iter());

        // Name and privacy should match those inside tokens
        assert_eq!(*function.name(), name);
        assert_eq!(*function.privacy().unwrap(), privacy.unwrap());
    }


    #[test]
    fn create_function() {
        let name = String::from("test_function");

        let tokens = vec![
            Token::FunctionStart,
            Token::FunctionName(name.clone()),
            Token::FunctionEnd
        ];

        let function = FunctionDef::new(&mut tokens.into_iter());

        // Name and privacy should match those inside tokens
        assert_eq!(*function.name(), name);
        assert!(function.privacy().is_none());
    }
}