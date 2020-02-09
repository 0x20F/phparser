pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }


    pub fn parse(&self, dirs: &[&str]) {
        // Start parsing after creating the parser

        // Walk through all items in each directory
        for dir in dirs {
            self.parse_dir(dir);
        }
    }


    pub fn parse_dir(&self, dir: &str) {
        // Find all files and go down the parse chain
    }


    pub fn parse_file(&self, file_path: String) {
        // New file struct -> pass path
    }
}









#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
