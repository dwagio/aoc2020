use std::fs;

fn main() {
    let numbers = read_input("dad_input");

    let invalid = find_invalid_number(&numbers, 25);
    let encryption_weakness = part_2(&numbers, invalid).unwrap();

    println!("{}", invalid);
    println!("{}", encryption_weakness);
}

fn read_input(fname: &str) -> Vec<u64> {
    let contents = fs::read_to_string(fname).expect("Failed to read input file");
    contents
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

fn find_invalid_number(numbers: &Vec<u64>, preamble_size: usize) -> u64 {
    let invalid_window = numbers.windows(preamble_size + 1).find(|window| {
        let current_number = window.get(preamble_size).unwrap();
        let mut preamble = vec![];
        preamble.extend_from_slice(window.get(0..preamble_size).unwrap());
        preamble.sort();

        let mut low = 0;
        let mut high = preamble_size - 1;

        while low < high {
            if preamble[low] + preamble[high] < *current_number {
                low += 1;
            } else if preamble[low] + preamble[high] > *current_number {
                high -= 1;
            } else {
                // current_number is valid for the given preamble
                return false;
            }
        }
        // Resolve to true if we find no matching sum
        true
    });

    match invalid_window {
        Some(window) => return window[preamble_size],
        None => panic!("Failed to find invalid number"),
    }
}

fn find_contiguous_sum(numbers: &Vec<u64>, target: u64) -> Option<Vec<u64>> {
    let mut low = 0;
    let mut high = 1;
    let mut sum: u64 = numbers[low..=high].iter().sum();

    while high < numbers.len() {
        if sum == target {
            let result = numbers[low..=high].to_vec();
            return Some(result);
        } else if sum > target {
            sum -= numbers[low];
            low += 1;
        } else {
            high += 1;
            sum += numbers[high];
        }
    }
    None
}

fn find_min_max(numbers: Vec<u64>) -> Option<u64> {
    numbers
        .iter()
        .min()
        .and_then(|min| numbers.iter().max().map(|max| min + max))
}

fn part_2(numbers: &Vec<u64>, target: u64) -> Option<u64> {
    find_contiguous_sum(numbers, target).and_then(find_min_max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_number() {
        let numbers = read_input("test_input");
        let invalid = find_invalid_number(&numbers, 5);
        assert_eq!(invalid, 127)
    }

    #[test]
    fn test_part_2() {
        let numbers = read_input("test_input");
        let results = part_2(&numbers, 127);
        assert_eq!(results.unwrap(), 62);
    }
}
