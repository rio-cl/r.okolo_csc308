fn main() {
    
    let mut numbers: Vec<i32> = (1..=20).collect();

    let is_even = |x: &i32| x % 2 == 0;

    let even_numbers: Vec<i32> = numbers.iter().cloned().filter(is_even).collect();

    println!("Even numbers: {:?}", even_numbers);
}

