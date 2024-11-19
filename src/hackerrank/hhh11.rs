fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    // Count apples that fall on the house
    let apples_on_house = apples
        .iter()
        .filter(|&&apple| {
            let landing_position = a + apple; // Calculate where the apple lands
            landing_position >= s && landing_position <= t
        })
        .count();

    // Count oranges that fall on the house
    let oranges_on_house = oranges
        .iter()
        .filter(|&&orange| {
            let landing_position = b + orange; // Calculate where the orange lands
            landing_position >= s && landing_position <= t
        })
        .count();

    // Print the results
    println!("{}", apples_on_house);
    println!("{}", oranges_on_house);
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read s and t (house boundaries)
    let first_line: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let s = first_line[0];
    let t = first_line[1];

    // Read a and b (positions of the trees)
    let second_line: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let a = second_line[0];
    let b = second_line[1];

    // Read number of apples and oranges (not used but required to match input structure)
    let _third_line: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read distances of apples
    let apples: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read distances of oranges
    let oranges: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Call the function
    count_apples_and_oranges(s, t, a, b, &apples, &oranges);
}
