fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades
        .iter()
        .map(|&grade| {
            if grade < 38 {
                grade // Grade is not rounded if less than 38
            } else {
                let next_multiple_of_5 = (grade + 4) / 5 * 5; // Find the next multiple of 5
                if next_multiple_of_5 - grade < 3 {
                    next_multiple_of_5 // Round up if the difference is less than 3
                } else {
                    grade // Otherwise, keep the grade as is
                }
            }
        })
        .collect()
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of grades
    let grades_count: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid input for grades count");

    // Read the grades
    let mut grades = Vec::new();
    for _ in 0..grades_count {
        let grade: i32 = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse()
            .expect("Invalid input for grades");
        grades.push(grade);
    }

    // Call the function
    let result = grading_students(&grades);

    // Print the result
    for grade in result {
        println!("{}", grade);
    }
}
