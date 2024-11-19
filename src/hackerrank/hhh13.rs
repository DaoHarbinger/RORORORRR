fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    // Helper function to check if all elements of `arr` are factors of `x`
    fn is_factor(x: i32, arr: &[i32]) -> bool {
        arr.iter().all(|&y| x % y == 0)
    }

    // Helper function to check if `x` is a factor of all elements in `arr`
    fn is_multiple(x: i32, arr: &[i32]) -> bool {
        arr.iter().all(|&y| y % x == 0)
    }

    // Get the maximum of `a` and minimum of `b`
    let max_a = *a.iter().max().unwrap();
    let min_b = *b.iter().min().unwrap();

    // Count all numbers between max_a and min_b that satisfy both conditions
    (max_a..=min_b)
        .filter(|&x| is_factor(x, a) && is_multiple(x, b))
        .count() as i32
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line: number of elements in `a` and `b`
    let first_line: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = first_line[0];
    let m = first_line[1];

    // Read the array `a`
    let a: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read the array `b`
    let b: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Compute the result
    let total = get_total_x(&a, &b);

    // Print the result
    println!("{}", total);
}
