use std::io::{self, BufRead, Write};

fn main() {
    print!("What is your name? ");
    io::stdout().flush().expect("Couldn't flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let name = input.trim_end_matches("\n");

    if name.len() == 0 {
        println!("No name entered :(, goodbye.");
    } else {
        println!("Hello, {}, nice to meet you!", name);
    }
}
