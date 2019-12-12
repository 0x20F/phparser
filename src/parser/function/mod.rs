use regex::{ Regex, Captures };
use md5::Digest;

use super::file::FileModel;



lazy_static! {
    static ref RE: Regex = Regex::new(r"(?m)(->|::)([A-Za-z_]+?)(\((?:.*?)\))").unwrap();
}



/// A parsed function blocks data
/// TODO: Some of these can be private so you can use setters with custom functionality
/// TODO: These shouldn't all be strings
pub struct FunctionModel {
    pub name        : String,
    pub visibility  : String,
    pub hash        : Digest,
    pub parent      : String,

    functions       : Vec<String>,
    function_refs   : Option<Vec<&'static FunctionModel>>
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
        let hash = md5::compute(&full_data);


        let mut functions: Vec<String> = vec![];

        for cap in RE.captures_iter(&full_data) {
            let fn_name = cap.get(2).map_or("", |m| m.as_str());
            functions.push(fn_name.to_string());
        }

        functions.sort_unstable();
        functions.dedup();


        FunctionModel {
            name,
            visibility,
            hash,
            parent: parent.name().to_string(),
            functions,
            function_refs: None
        }
    }
}