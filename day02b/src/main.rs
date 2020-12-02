use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read file!");

    let lines: Vec<&str> = contents.lines().collect();

    let regex = Regex::new(r"(\d+)-(\d+)\s([a-z]):\s([a-z]+)").unwrap();

    let results =  lines
        .iter()
        .filter(|line| {
            let captures = regex.captures(line).unwrap();
            let first_index = captures.get(1).map_or(0, |m| m.as_str().parse::<usize>().unwrap());
            let second_index = captures.get(2).map_or(0, |m| m.as_str().parse::<usize>().unwrap());
            let letter = captures.get(3).map_or("", |m| m.as_str());
            let password = captures.get(4).map_or("", |m| m.as_str());

            let letter_at_first_index = password.get(first_index-1..first_index).unwrap() == letter;
            let letter_at_second_index = password.get(second_index-1..second_index).unwrap() == letter;

            !(letter_at_first_index && letter_at_second_index) && (letter_at_first_index || letter_at_second_index)
        });

    println!("{}", results.count());
}
