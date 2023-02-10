use std::io;

fn main() {
    println!("Enter number: ");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("cannot read");

    let num: i64 = num
        .trim().parse()
        .expect("not i64");

    let mut vec = Vec::new();

    for i in 1..=num {
        if num % i == 0 {
            vec.push(i)
        }
    }
    println!("{:?}", vec);
}
