fn step(s: &mut String) -> &String{
    s.push('?');
    s
}

fn main(){
    let mut a = String::from("ok");
    let b = step(&mut a);
    let c = step(&mut a);
    println!("{a} {}", b.len() + c.len());
}
