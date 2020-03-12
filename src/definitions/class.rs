use crate::lexer::Token;


pub struct ClassDef {
    name: String,
}


impl ClassDef {
    pub fn new<I>(mut tokens: &mut I)
        where I: Iterator<Item = Token>
    {
        let mut token = tokens.next().unwrap();

        // Loop through tokens until ClassEnd is reached
        // This should all build a ClassDef object
        loop {
            match token {
                Token::ClassName(_, n) => println!("Theres a class name: {}", n),

                Token::FunctionStart(_) => Self::build_function(&mut tokens),

                Token::ClassEnd(_) => {
                    println!("Class end now");
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
    }


    fn build_function<I>(tokens: &mut I)
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
    }
}