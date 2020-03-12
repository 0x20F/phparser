use crate::lexer::Token;


pub struct FunctionDef {}


impl FunctionDef {
    pub fn new<I>(tokens: &mut I) -> Self
        where I: Iterator<Item = Token>
    {
        let mut token = tokens.next().unwrap();

        // Loop through tokens until FunctionEnd is reached
        // This should all build a FunctionDef object
        loop {
            match token {
                Token::FunctionName(_, n) => println!("Found function with name: {}", n),
                Token::FunctionPrivacy(_, p) => println!("And privacy: {}", p.unwrap()),
                Token::FunctionEnd(_) => {
                    println!("Function ends now");
                    break;
                },
                _ => ()
            }

            if let Some(t) = tokens.next() {
                token = t;
            } else {
                break;
            }
        }

        FunctionDef {}
    }
}