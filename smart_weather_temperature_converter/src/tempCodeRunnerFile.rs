use std::io;

fn main() {
    let mut temperature = String::new();
    let mut temp_type = String::new();

    println!("\n\nEnter the temperature you want to convert to");
    io::stdin().read_line(&mut temp_type).expect("Failed to read input");
    let temp_type = temp_type.trim().to_lowercase();

    if temp_type == "fahrenheit" {
        println!("\n\nEnter the temperature in celcius: ");
        io::stdin().read_line(&mut temperature).expect("Failed to read input");
        let temperature: f64 = temperature.trim().parse().expect("Please enter a valid number");

        let new_temp = temperature * (9.0 / 5.0) + 32.0;

        println!("Temperature: {:.1} °C", temperature);
        println!("Converted: {:.1} °F", new_temp);
    } else if temp_type == "celsius" {
        println!("\n\nEnter the temperature in Fahrenheit: ");
        io::stdin().read_line(&mut temperature).expect("Failed to read input");
        let temperature: f64 = temperature.trim().parse().expect("Please enter a valid number");

        let new_temp = (temperature - 32.0) * (5.0 / 9.0);

        println!("Temperature: {:.1} °F", temperature);
        println!("Converted: {:.1} °C", new_temp);
    } else {
        println!("Invalid option. Please enter either 'Fahrenheit' or 'Celsius'.");
    }

    
}
