use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read input file!");

    let empty_line_regex = Regex::new(r"[\n|\r\n]{3,}").unwrap();
    let passports: Vec<&str> = empty_line_regex.split(&contents).collect();

    let re = regex::Regex::new(r"\s+").unwrap();

    let count = passports
        .iter()
        .filter(|passport| {
            // Unsplit fields with their values. Formatted as "field:value".
            let fields_with_values: Vec<&str> = re.split(passport).map(|s| s.trim()).collect();

            // Field names only with cid filtered out
            let fields_minus_cid: Vec<&str> = fields_with_values
                .iter()
                .map(|field| field.split(":").next().unwrap())
                .filter(|field| field != &"cid")
                .collect();

            fields_minus_cid.len() >= 7
        })
        .collect::<Vec<&&str>>()
        .len();

    println!("{}", count);
}
