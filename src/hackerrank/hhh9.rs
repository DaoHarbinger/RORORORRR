fn time_conversion(s: &str) -> String {
    // Extract the AM/PM period
    let period = &s[s.len() - 2..];
    // Extract the time part without AM/PM
    let time = &s[..s.len() - 2];

    // Split the time into hours, minutes, and seconds
    let mut parts: Vec<String> = time.split(':').map(|part| part.to_string()).collect();
    let hour = parts[0].parse::<i32>().unwrap();

    // Convert the hour
    if period == "AM" {
        if hour == 12 {
            parts[0] = "00".to_string(); // Convert 12:XX:XX AM to 00:XX:XX
        }
    } else if period == "PM" {
        if hour != 12 {
            parts[0] = (hour + 12).to_string(); // Convert XX:XX:XX PM to (XX+12):XX:XX
        }
    }

    // Return the result in HH:MM:SS format
    parts.join(":")
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the input time in 12-hour format
    let s = lines.next().unwrap().unwrap();

    // Call the time_conversion function
    let result = time_conversion(&s);

    // Print the result in 24-hour format
    println!("{}", result);
}
