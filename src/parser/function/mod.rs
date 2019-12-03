/// A parsed function blocks data
/// TODO: Some of these can be private so you can use setters with custom functionality
/// TODO: These shouldn't all be strings
pub struct FunctionModel {
    pub name: String,
    pub visibility: String,
    pub hash: String,
    pub parent: String,
    pub params: Vec<String>,
    pub children: Vec<String>,
}


impl FunctionModel {
    
    /// Parse the given function data to compile a list of important
    /// info about a function
    /// 
    /// TODO: Make it happen
    pub fn new(data: &str) -> FunctionModel {

        FunctionModel {
            name: String::from(data),
            visibility: String::from(data),
            hash: String::from("12t809j39t1j3t12qwe12e12t23timorgkm"),
            parent: String::from("This/Very/Long/Path/To/A/file.php"),
            params: vec![],
            children: vec![]
        }
    }
    
    
    
    /// Gets the name of the file this function is in
    /// 
    /// # Example
    /// ```
    /// let block = FunctionBlock {
    ///     // Other params...
    ///     parent: "Path/To/File/Containing/This/file.php"
    /// };
    /// 
    /// block.filename_parent() // returns "file.php"
    /// ```
    pub fn filename_parent(&self) -> String {
        let vec: Vec<&str> = self.parent.split("/").collect();
        vec.last().unwrap().to_string()
    }
}