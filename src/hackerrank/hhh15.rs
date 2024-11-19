fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut count = 0;
    let length = s.len();
    for i in 0..=length - m as usize {
        if s[i..i + m as usize].iter().sum::<i32>() == d {
            count += 1;
        }
    }
    count
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the chocolate array
    let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the chocolate array
    let s: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read the day and month
    let first_multiple_input: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let d = first_multiple_input[0];
    let m = first_multiple_input[1];

    // Call the function
    let result = birthday(&s, d, m);

    // Print the result
    println!("{}", result);
}
