use std::io;
use std::mem;

fn sort_vector( mut vect: Vec<i32> ) -> Vec<i32> {
    for i in 0..vect.len() - 1 {
        for j in 1..vect.len() {
            mem::swap( &mut vect[i], &mut vect[j] );
        }
    }
    return vect;
}

fn main() {
    let mut vect = String::new();

    io::stdin()
        .read_line(&mut vect)
        .expect("could not read");

    let vect: Vec<i32> = vect.split_whitespace()
        .map( |s| s.parse().unwrap() )
        .collect();

    println!("{:?}", sort_vector( vect ));
}


