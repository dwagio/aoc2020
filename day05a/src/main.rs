use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read input file");
    let passes: Vec<&str> = contents.lines().collect();

    let max = passes.iter().map(|pass| find_seat_id(pass)).max().unwrap();

    println!("{}", max);
}

fn find_seat_id(input: &str) -> usize {
    let mut left = 0;
    let mut right = 7;
    let mut front = 0;
    let mut back = 127;

    input.trim().chars().for_each(|ch| {
        match ch {
            'F' => back = front + ((back - front) / 2),
            'B' => front = back - ((back - front) / 2),
            'R' => left = right - ((right - left) / 2),
            'L' => right = left + ((right - left) / 2),
            _ => {}
        }
    });

    front * 8 + right
}

// Wrote a test here because of integer division shenanigans
// Turns out Rust does floor integer division!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let pass = "FBFBBFFRLR";
        let expected_id = 357;

        let id = find_seat_id(pass);

        assert_eq!(id, expected_id);
    }
}