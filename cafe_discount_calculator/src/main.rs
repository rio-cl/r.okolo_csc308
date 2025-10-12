use std::io;

fn main() {
    let mut original_price = String::new();
    
    println!("\n\nEnter the price of the customer bill");
    io::stdin().read_line(&mut original_price).expect("Failed to read input");
    let original_price: f64 = original_price.trim().parse().expect("Please enter a valid number");

    if original_price >= 5000.0 && original_price < 10000.0 {
        let discount_rate = 0.10;
        let new_price = original_price * discount_rate;
        let final_price = original_price - new_price;

        println!("\n\nOriginal Bill: ₦{:.2}", original_price);
        println!("Discount Applied: 10%");
        println!("Final price: ₦{:.2}\n2", final_price);
    } else if original_price > 10000.0 {
        let discount_rate = 0.15;
        let new_price = original_price * discount_rate;
        let final_price = original_price - new_price;

        println!("\nOriginal Bill: ₦{:.2}", original_price);
        println!("Discount Applied: 15%");
        println!("Final price: ₦{:.2}\n", final_price);
    } else {
        println!("\nOriginal Bill: ₦{:.2}", original_price);
        println!("Discount Applied: No applied discount");
        println!("Final price: ₦{:.2}\n", original_price);
    }
}
