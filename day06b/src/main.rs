use regex::Regex;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read file!");

    let empty_line_regex = Regex::new(r"[\n|\r\n]{3,}").unwrap();
    let groups: Vec<&str> = empty_line_regex.split(&contents).collect();

    let count = groups
        .iter()
        .map(|group| {
            let lines: Vec<&str> = group.lines().collect();

            let intersections = lines.iter().fold(None, |acc: Option<HashSet<char>>, line| {
                let mut questions = HashSet::new();
                line.chars().for_each(|c| {
                    questions.insert(c);
                });
                if acc.is_none() {
                    return Some(questions);
                } else {
                    let unioned: HashSet<char> = acc
                        .unwrap()
                        .intersection(&questions.clone())
                        .map(|c| *c)
                        .collect();
                    return Some(unioned);
                }
            });

            return intersections.unwrap().len();
        })
        .fold(0, |acc, val| acc + val);

    println!("{}", count);
}
