use std::io;

fn main() {

    println!("\n\nEnter the Number of KiloWatts used by the customer: ");
    let mut customer_watts = String::new();
    io::stdin().read_line(&mut customer_watts).expect("Failed to read input");
    let customer_watts: i32 = customer_watts.trim().parse().expect("Please enter a valid number");

    if customer_watts > 100 && customer_watts <= 200 {

        let rate = 25;
        let price = customer_watts * rate;

        println!("\nCustomer KW: {}kw", customer_watts);
        println!("Customer Rate per Unit: ₦{}",rate);
        println!("Customer Price: ₦{}",price);
    } else if customer_watts > 200 {
        let rate = 30;
        let price = customer_watts * rate;

        println!("\nCustomer KW: {}kw", customer_watts);
        println!("Customer Rate per Unit: ₦{}",rate);
        println!("Customer Price: ₦{}",price);
    }else{
        let rate = 20;
        let price = customer_watts * rate;

        println!("\nCustomer KW: {}kw", customer_watts);
        println!("Customer Rate per Unit: ₦{}",rate);
        println!("Customer Price: ₦{}",price);
    }
}
