use std::io;

fn is_prime( num: i32 ) -> bool {
    let mut out = true;
    let mut i = 2;
    while i*i <= num {
        if num % i == 0 {
            out = false;
        }
        i += 1;
    }
    return out;
}

fn main() {
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("could not read");

    let num: i32 = num
        .trim().parse()
        .expect("not i32");

    if is_prime( num ) == false {
        println!("Not a prime");
    } else {
        println!("Is a prime");
    }
}
