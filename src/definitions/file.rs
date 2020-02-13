pub struct FileDef {
    pub path: String,
    pub name: String
}


impl FileDef {
    pub fn new(path: String) -> FileDef {

        let name = FileDef::parse_name(&path);

        FileDef {
            path,
            name
        }
    }





    fn parse_name(path: &String) -> String {
        let pieces: Vec<&str> = path
            .split(&['/', '\\'][..])
            .collect();

        pieces.last().unwrap().to_string()
    }
}