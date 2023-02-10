use std::io;

fn main() {
    println!("Enter a number: ");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Could not read line");
    let n: i64 = n.trim().parse().expect("Not an i64");

    if n % 2 == 0 {
        if n % 4 == 0 {
            println!("Even but divisible by 4");
        } else {
            println!("Even but not divisible by 4");
        }
    } else {
        println!("It is an odd number");
    }
    
    let mut check = String::new();
    let mut num = String::new();
    
    println!("Enter the check value: ");
    io::stdin()
        .read_line(&mut check)
        .expect("Could not read line");

    println!("Enter the num value: ");
    io::stdin()
        .read_line(&mut num)
        .expect("Could not read line");

    let check: i64 = check.trim().parse().expect("Not i64");
    let num: i64 = num.trim().parse().expect("Not i64");

    if num % check == 0 {
        println!("Check evenly divides num");
    } else {
        println!("Check does not divide num");
    }

}
