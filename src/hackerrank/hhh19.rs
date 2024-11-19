use std::collections::HashMap;

fn sockMerchant(_n: i32, ar: &[i32]) -> i32 {
    let mut color_counts = HashMap::new();
    let mut pairs = 0;

    // Count the socks of each color
    for &sock in ar {
        let count = color_counts.entry(sock).or_insert(0);
        *count += 1;

        // Add to pairs when two socks are found
        if *count == 2 {
            pairs += 1;
            *count = 0; // Reset after forming a pair
        }
    }

    pairs
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of socks
    let _n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the colors of the socks
    let ar: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Calculate the number of pairs
    let result = sockMerchant(_n, &ar);

    // Output the result
    println!("{}", result);
}
