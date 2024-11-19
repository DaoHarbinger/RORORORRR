fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = (n - i) as usize; // Number of spaces
        let hashes = i as usize; // Number of hashes
        println!("{}{}", " ".repeat(spaces), "#".repeat(hashes));
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the staircase
    let n: i32 = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid input for n");

    // Call the staircase function
    staircase(n);
}
