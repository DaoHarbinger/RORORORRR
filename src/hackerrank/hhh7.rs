fn mini_max_sum(arr: &[i32]) {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    // Calculate the minimum and maximum sums
    let min_sum: i64 = sorted_arr.iter().take(4).map(|&x| x as i64).sum();
    let max_sum: i64 = sorted_arr.iter().skip(1).map(|&x| x as i64).sum();

    // Print the results
    println!("{} {}", min_sum, max_sum);
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the array
    let arr: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    // Call the function
    mini_max_sum(&arr);
}
