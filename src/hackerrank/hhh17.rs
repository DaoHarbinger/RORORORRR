fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut counts = [0; 5]; // Array to count occurrences of five bird types

    // Count occurrences for each bird type
    for &bird in arr {
        counts[(bird - 1) as usize] += 1;
    }

    // Find the bird type with the highest count (lowest ID if tied)
    let mut max_count = 0;
    let mut bird_id = 1;

    for (i, &count) in counts.iter().enumerate() {
        if count > max_count {
            max_count = count;
            bird_id = i as i32 + 1;
        }
    }

    bird_id
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let _arr_count: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let arr: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    println!("{}", result);
}
