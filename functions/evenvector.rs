use std::io;

fn even_number_filter( numbers: Vec<i32> ) -> Vec<i32> {
    /* Initialize an empty vector for pushing
     * even numbers*/
    let mut filtered = Vec::new();
    /* Iterate the original vector of numbers*/
    for n in numbers {
        /* check for even */
        if n % 2 == 0 {
            filtered.push(n)
        }
    }
    return filtered;
}

fn main() {
    println!("Enter the list of numbers:");
    let mut list = String::new();
    io::stdin()
        .read_line(&mut list)
        .expect("could not read");

    let list = list.split_whitespace()
        .map( |s| s.parse().unwrap() )
        .collect();

    println!("The even filtered list: ");
    println!("{:?}", even_number_filter(list) );
}
