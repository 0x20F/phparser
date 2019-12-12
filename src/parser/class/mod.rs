use std::sync::{ Arc, RwLock };



pub struct ClassModel {
    name: String,
    namespace: String, // Whatever\Whatever\ClassName
}

impl ClassModel {
    pub fn new() -> Arc<RwLock<ClassModel>> {

        Arc::new(RwLock::new( ClassModel {
            name: String::from("Aye"),
            namespace: String::from("Ayo")
        }))
    }
}