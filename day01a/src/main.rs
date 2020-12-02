use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read file!");

    let mut nums: Vec<i32> = contents.lines().map(|l| l.parse().unwrap()).collect();

    nums.sort();

    let mut l = 0;
    let mut r = nums.len() - 1;

    while l < r {
        if nums[l] + nums[r] == 2020 {
            println!("{}", nums[l] * nums[r]);
            return;
        } else if nums[l] + nums[r] < 2020 {
            l += 1;
        } else {
            r -= 1;
        }
    }
}
