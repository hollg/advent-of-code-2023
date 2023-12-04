use std::env;
use std::fs;

const INPUT_FILE_PATH: &str = "./input.txt";

fn main() {
    const RADIX: u32 = 10;
    let mut calibration_values: Vec<i32> = Vec::new();

    for line in fs::read_to_string(INPUT_FILE_PATH).unwrap().lines() {
        let mut calibration_value_as_string: String = String::new();
        let digit_chars: Vec<char> = line.chars().filter(|c| c.is_digit(RADIX)).collect();
        let first_digit = digit_chars[0];
        let last_digit = digit_chars[digit_chars.len() - 1];

        calibration_value_as_string.push(first_digit);
        calibration_value_as_string.push(last_digit);

        calibration_values.push(calibration_value_as_string.parse::<i32>().unwrap());
    }

    let total = calibration_values.iter().sum::<i32>();
    println!("Total: {}", total)
}
