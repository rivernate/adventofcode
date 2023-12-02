use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};
use regex::{Regex, Captures};

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let sum = lines
            .filter_map(|line| line.ok())
            .map(|line| extract_numbers(line))
            .filter_map(|line| get_calibration_value(line).ok())
            .sum::<i64>();

        println!("sum: {}", sum);
    }
}

fn get_calibration_value(digits: String) -> Result<i64, &'static str> {
    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;

    for c in digits.chars() {
        if c.is_digit(10) {
            if first_digit.is_none() {
                first_digit = Some(c);
                last_digit = Some(c);
            } else {
                last_digit = Some(c);
            }
        }
    }

    if let (Some(first), Some(last)) = (first_digit, last_digit) {
        let concat = format!("{}{}", first, last);
        concat
            .parse::<i64>()
            .map_err(|_| "Failed to parse calibration value")
    } else {
        Err("Failed to get calibration value")
    }
}

fn extract_numbers(input: String) -> String {
    let number_map: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_digit(10) {
            output.push(c);
            chars.next();
            continue;
        }

        // Get the next 5 chars starting from where we are
        // since all numbers spelled out are less then 5 chars
        // and convert it to a string
        let next_chars = peek_ahead_chars(&mut chars, 5);
        let full_word: String = next_chars.iter().collect();

        for (word, &number) in &number_map {
            if full_word.starts_with(word) {
                output.push_str(number);
                chars.next();
                continue;
            }
        }
        chars.next();
        continue;
    }
    output
}

fn peek_ahead_chars<I>(iter: &mut I, steps: usize) -> Vec<I::Item>
where
    I: Iterator + Clone,
    I::Item: Copy,
{
    let cloned_iter = iter.clone();
    cloned_iter.take(steps).collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers_to_digits() {
        let test_cases = vec![
            ("5ffour295", "54295"),
            ("cbtnktlrksevenone4onesevensevensvvxjjgrx", "714177"),
        ];

        for (input, expected) in test_cases {
            let result = extract_numbers(input.to_string());
            assert_eq!(result, expected);
        }
    }
}
