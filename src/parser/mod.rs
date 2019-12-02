/// A parsed function blocks data
pub struct FunctionBlock {
    name: String,
    visibility: String,
    params: Vec<String>,
    hash: String,
    children: Vec<String>,
    parent: String
}

impl FunctionBlock {
    /// Gets the name of the file this function is in
    /// 
    /// # Example
    /// ```
    /// let block = FunctionBlock {
    ///     name: String::from("getPancakes"),
    ///     visibility: String::from("public"),
    ///     params: ["a", "b", "c"],
    ///     hash: md5::compute(b"this is all the function data into one string or something"),
    ///     children: ["getFlour", "getMilk", "getSugar"],
    ///     parent: "Path/To/File/Containing/This/file.php"
    /// };
    /// 
    /// block::filename_parent() // returns "file.php"
    /// ```
    pub fn filename_parent(&self) -> String {
        let vec: Vec<&str> = self.parent.split("/").collect();
        vec.last().unwrap().to_string()
    }
}



pub fn run(dir: &str) -> FunctionBlock {
    println!("Indexing the following paths ({})", dir);

    FunctionBlock {
        name: String::from(dir),
        visibility: String::from(dir),
        params: vec![],
        hash: String::from("12t809j39t1j3t12qwe12e12t23timorgkm"),
        children: vec![],
        parent: String::from("This/Very/Long/Path/To/A/file.php")
    }
}


pub fn files() {

}