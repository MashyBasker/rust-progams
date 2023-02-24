use std::io;

/* subroutine for reversing the string*/
fn reverse_word( word: String ) -> String {

    /* .chars() function converts a collection into its characters*/
    /* .rev() function returns a reverse iterator */
    /* .collect() function returns a collection of the iterators */
    return word.chars().rev().collect()
}

fn main() {
    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Could not read");

    println!("Original word: {}", word);
    println!("Reversed word: {}", reverse_word(word));

}
