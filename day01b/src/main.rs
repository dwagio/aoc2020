use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read file!");

    let mut nums: Vec<i32> = contents.lines().map(|l| l.parse().unwrap()).collect();

    nums.sort();

    for i in 0..nums.len() - 2 {
        let mut l = i + 1;
        let mut r = nums.len() - 1;

        while l < r {
            if nums[i] + nums[l] + nums[r] == 2020 {
                println!("{}", nums[i] * nums[l] * nums[r]);
                return;
            } else if nums[i] + nums[l] + nums[r] < 2020 {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }
}
