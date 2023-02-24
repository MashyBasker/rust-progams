use std::io;

fn get_sum( a: i32, b: i32 ) -> i32 {
    return a + b;
}

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    io::stdin()
        .read_line(&mut num1)
        .expect("Could not read");

    io::stdin()
        .read_line(&mut num2)
        .expect("could not read");
    
    let num1: i32 = num1.trim().parse().expect("Not i32");
    let num2: i32 = num2.trim().parse().expect("Not i32");

    println!("Sum of the numbers: {}", get_sum(num1, num2));

}
