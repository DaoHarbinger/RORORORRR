fn divisibleSumPairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;

    for i in 0..n as usize {
        for j in i + 1..n as usize {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_multiple_input: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let n = first_multiple_input[0];
    let k = first_multiple_input[1];

    let ar: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = divisibleSumPairs(n, k, &ar);

    println!("{}", result);
}
