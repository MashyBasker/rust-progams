use std::io;

fn main() {
    println!("Enter the array as space seperated integers: ");
    let mut list = String::new();
    io::stdin()
        .read_line(&mut list)
        .expect("could not read");

    let  vec: Vec<i64> = list.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Enter the number for comparing: ");
    let mut cmp = String::new();
    io::stdin()
        .read_line(&mut cmp)
        .expect("could not read");

    let cmp: i64 = cmp.trim().parse().expect("not i64");
    
    let mut empty_vec = Vec::new();

    for num in vec {
        if num >= cmp {
            empty_vec.push(num);
        }
    }

    println!("The final array: ");
    println!("{:?}", empty_vec);

}
