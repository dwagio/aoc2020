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
            let low_bound = captures.get(1).map_or(0, |m| m.as_str().parse::<usize>().unwrap());
            let high_bound = captures.get(2).map_or(0, |m| m.as_str().parse::<usize>().unwrap());
            let letter = captures.get(3).map_or("", |m| m.as_str());
            let password = captures.get(4).map_or("", |m| m.as_str());

            let char_count = password.matches(letter).count();

            char_count >= low_bound && char_count <= high_bound 
        });

    println!("{}", results.count());
}
