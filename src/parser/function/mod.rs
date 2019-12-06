extern crate md5;


use md5::Digest;

use super::file::FileModel;


/// A parsed function blocks data
/// TODO: Some of these can be private so you can use setters with custom functionality
/// TODO: These shouldn't all be strings
pub struct FunctionModel {
    pub name: String,
    pub visibility: String,
    pub hash: Digest,
    pub parent: String,
    pub children: Vec<String>,
}


impl FunctionModel {
    
    /// Parse the given function data to compile a list of important
    /// info about a function
    pub fn new(data: Vec<String>, parent: &FileModel) -> FunctionModel {

        // Declaration is always first line
        let declaration = data.first().unwrap().to_owned();
        let full_data = data.join("\n");

        // public function fancyName(parameters)
        let mut keywords: Vec<&str> = declaration
            .split(&[' ', '('][..])
            .collect();
        keywords.retain(|&word| word.len() > 0);

        let visibility = String::from(keywords.first().unwrap().to_owned());
        let name = String::from(keywords.get(2).unwrap().to_owned());
        let hash = md5::compute(full_data);


        FunctionModel {
            name,
            visibility,
            hash,
            parent: parent.name().to_string(),
            children: vec![]
        }
    }
}