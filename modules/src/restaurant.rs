pub struct Lunch {
    pub soup: String,
}

impl Lunch {
    pub fn new(soup: &str) -> Lunch {
        Lunch {
            soup: String::from(soup),
        }
    }
}