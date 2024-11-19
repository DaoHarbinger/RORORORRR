use std::io::{self, BufRead};

fn pageCount(n: i32, p: i32) -> i32 {
    let front_turns = p / 2;
    let back_turns = (n / 2) - (p / 2);
    front_turns.min(back_turns)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n: i32 = stdin_iterator.next().unwrap().unwrap().trim().parse().unwrap();
    let p: i32 = stdin_iterator.next().unwrap().unwrap().trim().parse().unwrap();

    let result = pageCount(n, p);
    println!("{}", result);
}
