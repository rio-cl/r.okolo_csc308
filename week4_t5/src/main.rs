use std::io;

#[derive(Debug)]
struct Evaluation {
    student: String,
    score: f64,
}

impl Evaluation{
    fn new(student: String, score: f64) -> Self {
        Self{
            student,
            score
        }
    }

    fn pass(&self){
        if self.score >= 40.0 {
            println!("{} has passed this course", self.student)
        } else {
            println!("{} student has not passed this course", self.student)
        }
    }
}

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter students name: ");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    let student = input1.trim().to_string();

    println!("Enter students Score: ");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");
    let score: f64 = input2
        .trim()
        .parse()
        .expect("Please enter a valid number for score.");

    let student_profile = Evaluation::new(student, score);

    student_profile.pass();
    




}
