use std::io;

fn main() {
    // TODO: Your code here
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", input);
}
