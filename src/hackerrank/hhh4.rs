fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..n {
        primary_diagonal_sum += arr[i][i]; // Primary diagonal element
        secondary_diagonal_sum += arr[i][n - 1 - i]; // Secondary diagonal element
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs() // Absolute difference
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Reading the size of the matrix
    let n: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid input for n");

    // Reading the matrix
    let mut arr: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let row: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid input for matrix element"))
            .collect();
        arr.push(row);
    }

    // Calculate the diagonal difference
    let result = diagonal_difference(&arr);

    // Print the result
    println!("{}", result);
}
