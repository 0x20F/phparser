pub struct FileDef {
    pub path: String
}



impl FileDef {
    pub fn new(path: String) -> FileDef {

        FileDef {
            path
        }
    }
}