use std::env;
use std::num::ParseIntError;
use std::cmp;

struct TribonacciError(String);

fn main() {
    let args: Vec<String> = env::args().collect();
    let error_message = String::from("Please enter a valid size");

    let size = match args.get(1) {
        Some(s) => s.parse::<usize>(),
        None => Ok(10),
    };

    if let Err(e) = compute_tribonacci(size, error_message) {
        println!("Error: {}", e.0)
    }
}

/// Computes the tribonacci sequence of a given size
/// Prints the sequence, and its sum
fn compute_tribonacci(
    size: Result<usize, ParseIntError>,
    // The error message your function should return
    // inside the `TribonacciError` struct
    error_msg: String,
) -> Result<(), TribonacciError> {

    let size = size.map_err(|_| TribonacciError(error_msg))?;
    let mut seq: Vec<u128> = Vec::new();
    // let mut sum: u128 = 0;
    if size > 0 {
        // sum += cmp::min(size, 3) as u128;
        seq.append(&mut vec![1; cmp::min(size, 3)]);
    }
    
    for i in 3..size {
        let next = seq[i - 1] + seq[i - 2] + seq[i - 3];
        // sum += next;
        seq.push(next);
    }
    let sum: u128 = seq.iter().sum();
    println!("Values: {:?}", seq);
    println!("");
    println!("Sum: {}", sum);

    Ok(())
}
