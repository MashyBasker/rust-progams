use std::io;

/* subroutine for finding longest string */ 
fn longest_string( string_list: Vec<String> ) -> String {
    let mut w = String::new();
    let mut n = 0;
    /* the .len() method gets the 
     * length of a String*/
    for s in string_list {
        if s.len() >= n { w = s.clone(); n = s.len(); }
    }
    return w;
}

fn main() {
    let mut words = String::new();
    io::stdin()
        .read_line(&mut words)
        .expect("could not read");
    
    /* Convert string to vector of String */
    let words: Vec<String> = words
        .split_whitespace()
        .map( |s| s.to_string() )
        .collect();

    println!("The longest word: {}", longest_string(words));
}
