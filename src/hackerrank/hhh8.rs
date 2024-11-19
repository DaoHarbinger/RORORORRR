fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let max_height = *candles.iter().max().unwrap(); // Find the maximum height
    candles.iter().filter(|&&height| height == max_height).count() as i32 // Count how many candles have this height
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of candles (not necessary to use)
    let _candles_count: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid input for candles count");

    // Read the heights of candles
    let candles: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input for candle height"))
        .collect();

    // Call the function and print the result
    let result = birthday_cake_candles(&candles);
    println!("{}", result);
}
