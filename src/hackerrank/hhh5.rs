fn plus_minus(arr: &[i32]) {
    let n = arr.len() as f64;
    let mut positive_count = 0;
    let mut negative_count = 0;
    let mut zero_count = 0;

    for &num in arr {
        if num > 0 {
            positive_count += 1;
        } else if num < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    // Print ratios with 6 decimal places
    println!("{:.6}", positive_count as f64 / n);
    println!("{:.6}", negative_count as f64 / n);
    println!("{:.6}", zero_count as f64 / n);
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let _n: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid input for n");

    // Read the array elements
    let arr: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input for array element"))
        .collect();

    // Call the function
    plus_minus(&arr);
}
