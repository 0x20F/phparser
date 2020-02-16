use super::definitions::{FileStream};




pub struct Token {
    token: TokenType,
    line: u64,
    symbol: String
}


pub enum TokenType {
    NotFound,

    Namespace,
    Use,

    ClassStart,
    ClassEnd
}



pub struct Lexer {}


impl Lexer {
    pub fn tokenize(stream: &mut FileStream) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        // Rewind


        tokens
    }
}