use std::io;
fn main() {
    let pattern_string = std::env::args()
        .nth(1)
        .expect("missing required command-line argument: <pattern>");

    let pattern = &pattern_string;

    // TODO: Replace the following with your code:
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if input.contains(pattern) {
            print!("{}", input);
        }

        if input.is_empty() {
            break;
        } 
    }
}
