use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read file!");

    let nums: HashSet<i32> = contents.lines().map(|l| l.parse().unwrap()).collect();

    for num in &nums {
        if nums.contains(&(2020 - num)) {
            println!("{}", (2020 - num) * num);
            return;
        }
    }
}
