struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn is_old(&self) -> bool {
        self.age > 32
    }
}

fn main() {
    let p = Person {
        name: String::from("Ryan Dunn"),
        age: 55
    };

    if p.is_old() {
        println!("{} is old!", p.name);
    } else {
        println!("{} is not old!", p.name);
    }

    let plus_two = |x| {
        let mut result: i32 = x;
    
        result += 1;
        result += 1;
    
        result
    };
}