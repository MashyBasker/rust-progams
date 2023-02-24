use std::io;

/* integer sum subroutine */ 
fn vec_sum( integers: Vec<i32> ) -> i32 {
    let mut sum: i32 = 0;
    for i in integers {
        sum += i;
    }
    return sum;
}

fn main() {
    println!("Provide the integers: ");
    let mut vector = String::new();

    io::stdin()
        .read_line(&mut vector)
        .expect("could not read");
    
    /* convert string to vector */ 
    let vector: Vec<i32> = vector.split_whitespace()
                                     .map(|s| s.parse().unwrap())
                                     .collect();
    println!("Vector sum: {}", vec_sum(vector));

}
