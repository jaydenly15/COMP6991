fn main() {
    for i in 1..=100 {
        let three = i % 3 == 0;
        let five = i % 5 == 0;

        if three && five {
            println!("FizzBuzz");
        } else if three {
            println!("Fizz");
        } else if five {
            println!("Buzz");
        } else {
            println!("{i}");
        }
    }
}
