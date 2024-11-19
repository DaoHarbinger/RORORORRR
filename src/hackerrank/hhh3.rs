use std::io::{self, BufRead};

fn aVeryBigSum(ar: &[i64]) -> i64 {
    // Sum up all the elements in the array
    ar.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements in the array
    let _n: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid input for n");

    // Read the array elements
    let ar: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input for array element"))
        .collect();

    // Call the function to calculate the sum
    let result = aVeryBigSum(&ar);

    // Print the result
    println!("{}", result);
}
