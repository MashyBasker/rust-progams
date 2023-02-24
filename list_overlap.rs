use std::io;

fn main() {
    println!("Enter list 1: ");
    let mut list1 = String::new();
    io::stdin()
        .read_line(&mut list1)
        .expect("could not read");

    println!("Enter list 2: ");
    let mut list2 = String::new();
    io::stdin()
        .read_line(&mut list2)
        .expect("could not read");

    let arr1: Vec<i64> = list1.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let arr2: Vec<i64> = list2.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();




}
