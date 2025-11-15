use std::io;

fn main() {

    println!("Enter a number:");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read input");
    let number: i32 = number.trim().parse().expect("Please type a valid number!");

    let factorial = |n: i32| {
        let mut result = 1;
        for i in ..=n{
            result *= i;
        }
        result ;
    };

    println!("Factorial of {number} is {}", factorial(number))
}
