pub struct ClassDef {
    name: String,
}

impl ClassDef {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}