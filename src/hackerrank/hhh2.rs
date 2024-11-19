fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut alice_score = 0;
    let mut bob_score = 0;

    for i in 0..a.len() {
        if a[i] > b[i] {
            alice_score += 1;
        } else if a[i] < b[i] {
            bob_score += 1;
        }
    }

    vec![alice_score, bob_score]
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let a: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    let b: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    let result = compare_triplets(&a, &b);

    println!("{} {}", result[0], result[1]);
}
