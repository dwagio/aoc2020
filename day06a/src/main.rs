use regex::Regex;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read file!");

    let empty_line_regex = Regex::new(r"[\n|\r\n]{3,}").unwrap();
    let groups: Vec<&str> = empty_line_regex.split(&contents).collect();

    let count = groups.iter().fold(0, |acc, group| {
        let mut questions = HashSet::new();
        group.chars().for_each(|c| {
            if c.is_alphabetic() {
                questions.insert(c);
            }
        });

        return acc + questions.len();
    });

    println!("{}", count);
}
