pub struct Breakfast {
    pub bread: String,
}

impl Breakfast {
    pub fn new(bread: &str) -> Breakfast {
        Breakfast {
            bread: String::from(bread),
        }
    }
}