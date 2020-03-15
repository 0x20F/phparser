use crate::lexer::Token;

pub trait ExtractTokens {
    fn take(&mut self, token: Token);
}