fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        // If velocities are equal, check if they start at the same position
        if x1 == x2 {
            return "YES".to_string();
        } else {
            return "NO".to_string();
        }
    }

    // Calculate the time when they might meet
    let t = (x2 - x1) as f64 / (v1 - v2) as f64;

    // They meet if time is positive and an integer
    if t > 0.0 && t.fract() == 0.0 {
        return "YES".to_string();
    } else {
        return "NO".to_string();
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input values
    let first_line: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let x1 = first_line[0];
    let v1 = first_line[1];
    let x2 = first_line[2];
    let v2 = first_line[3];

    // Call the function and print the result
    let result = kangaroo(x1, v1, x2, v2);
    println!("{}", result);
}
