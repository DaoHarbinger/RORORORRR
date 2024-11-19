fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut max_score = scores[0];
    let mut min_score = scores[0];
    let mut max_breaks = 0;
    let mut min_breaks = 0;

    for &score in scores.iter().skip(1) {
        if score > max_score {
            max_score = score;
            max_breaks += 1;
        }
        if score < min_score {
            min_score = score;
            min_breaks += 1;
        }
    }

    vec![max_breaks, min_breaks]
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of games (not used in the logic)
    let _ = lines.next().unwrap().unwrap();

    // Read the array of scores
    let scores: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Call the function to calculate records
    let result = breaking_records(&scores);

    // Print the result
    println!("{} {}", result[0], result[1]);
}
