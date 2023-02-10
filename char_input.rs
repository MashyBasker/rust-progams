use std::io;

fn main() {
    println!("Enter your name: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Could not read line");

    println!("Enter your age: ");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Could not read line");
    let sanitized_age: i32 = age.trim().parse().expect("Not an integer");

    println!("Specify the number of copies: ");
    let mut copies = String::new();
    io::stdin()
        .read_line(&mut copies)
        .expect("Could not read line");
    let mut sanitized_copies: i64 = copies.trim().parse().expect("Not an integer");

    let year = 2023 + (100 - sanitized_age) - 1;
    
    
    while sanitized_copies != 0 {
        println!("Hey {}! You will turn 100 in the year {}", name, year);
        sanitized_copies -= 1;
    }
    println!("Thanks!");

}
