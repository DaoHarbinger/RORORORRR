fn bon_appetit(bill: &[i32], k: usize, b: i32) {
    // Calculate the actual amount Anna should pay
    let total_cost: i32 = bill.iter().sum();
    let anna_share = (total_cost - bill[k]) / 2;

    if b == anna_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b - anna_share);
    }
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

    let n = first_multiple_input[0] as usize;
    let k = first_multiple_input[1] as usize;

    let bill: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let b: i32 = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    bon_appetit(&bill, k, b);
}
