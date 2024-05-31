use std::fmt::Display;

struct Point<T: Display, U: Display> {
    x: T,
    y: U,
}

impl<T: Display, U: Display> Point<T, U> {
    fn print_values(&self) {
        println!("({}, {})", self.x, self.y);
    }

    fn mixup<X, Y>(self, other: Point<X, Y>) -> Point<T, Y> 
    where
        X: Display,    
        Y: Display,
    {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p1 = Point {
        x: 1,
        y: 2
    };

    let p2: Point<f32, f32> = Point {
        x: 5.0,
        y: 8.0
    };

    p1.print_values();
    p2.print_values();
    
    let dist: f32 = p2.distance_from_origin();
    println!("{}", dist);

    let p3 = p1.mixup(p2);
    p3.print_values();
}
