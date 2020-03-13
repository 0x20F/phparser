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









#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_method() {
        let name = String::from("test_function");
        let privacy = Some(String::from("protected"));

        let tokens = vec![
            Token::FunctionStart(1),
            Token::FunctionName(1, name.clone()),
            Token::FunctionPrivacy(1, privacy.clone()),
            Token::FunctionEnd(1)
        ];

        let function = FunctionDef::new(&mut tokens.into_iter());

        // Name and privacy should match those inside tokens
        assert_eq!(function.name, name);
        assert_eq!(function.privacy.unwrap(), privacy.unwrap());
    }


    #[test]
    fn create_function() {
        let name = String::from("test_function");

        let tokens = vec![
            Token::FunctionStart(1),
            Token::FunctionName(1, name.clone()),
            Token::FunctionEnd(1)
        ];

        let function = FunctionDef::new(&mut tokens.into_iter());

        // Name and privacy should match those inside tokens
        assert_eq!(function.name, name);
        assert!(function.privacy.is_none());
    }
}