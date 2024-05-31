mod protocol;
mod restaurant;

fn main() {
    let b = protocol::Breakfast::new("Baguette");
    println!("{}", b.bread);

    let l = restaurant::Lunch::new("French Onion");
    println!("{}", l.soup);
}
