mod token;

pub use token::Token;




pub struct Lexemes<'a> {
    code: &'a str,
    special: &'a [char]
}


impl<'a> Lexemes<'a> {
    pub fn from(code: &'a str) -> Self {
        Self {
            code,
            special: &['(', ')', '{', '}', ';', '=']
        }
    }

    fn update(&mut self, margin: usize) {
        self.code = &self.code[margin..].trim();
    }

    fn is_special(&self, c: &char) -> bool {
        self.special.contains(c)
    }
}


impl<'a> Iterator for Lexemes<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token = None;

        if self.code.is_empty() {
            return None;
        }

        let first = self.code.chars().next().unwrap_or_default();

        // If the character is special,
        // return only the first occurrence of that
        // character
        if self.is_special(&first) {
            token = Some(&self.code[..1]);
            self.update(1);

            return token;
        }

        // If the character wasn't special,
        // get everything until a space or until
        // we hit another special character
        let rest = self.code.char_indices()
            .take_while(|(_, c)| !c.is_whitespace() && !self.is_special(&c))
            .last()
            .map(|(i, c)| i + c.len_utf8())
            .unwrap_or_default();


        token = Some(&self.code[..rest]);
        self.update(rest);

        token
    }
}








#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex() {
        let code = "class A { function b() {} } $a = 50; $b = Container::class;";
        let mut tokens = Lexemes::from(code);

        assert_eq!(tokens.count(), 21);
    }
}