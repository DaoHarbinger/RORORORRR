use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn simpleArraySum(ar: &[i32]) -> i32 {
    ar.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();


    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();


    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();


    let result = simpleArraySum(&ar);

    writeln!(&mut fptr, "{}", result).ok();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_array_sum_multiple_cases() {
        let ar1 = vec![1, 2, 3, 4, 10, 11];
        let expected_sum1 = 31;
        assert_eq!(simpleArraySum(&ar1), expected_sum1);


        let ar2 = vec![];
        let expected_sum2 = 0;
        assert_eq!(simpleArraySum(&ar2), expected_sum2);


        let ar3 = vec![-1, -2, -3, -4];
        let expected_sum3 = -10;
        assert_eq!(simpleArraySum(&ar3), expected_sum3);


        let ar4 = vec![100, 200, 300];
        let expected_sum4 = 600;
        assert_eq!(simpleArraySum(&ar4), expected_sum4