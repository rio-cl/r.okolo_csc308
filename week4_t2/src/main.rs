use std::f64::consts::PI;
use std::io;

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {

    fn new(radius: f64) -> Self {
        Self { radius }
    }

    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}


fn main() {
    let mut input = String::new();

    println!("Enter the radius of the circle: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let radius: f64 = input.trim().parse().expect("Please enter a valid number");

    let circle = Circle::new(radius);

    println!("\n--- Circle Details ---");
    println!("Radius: {:.2}", circle.radius);
    println!("Area: {:.2}", circle.area());
}
