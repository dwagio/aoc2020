use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read input file!");

    let contents = contents.lines().collect::<Vec<&str>>();

    let mut multiplied_trees: i64 = 1;
    let deltas = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];

    for current_slope in 0..deltas.len() {
        let mut num_trees = 0;
        let mut x_index = 0;
        let mut y_index = 0;

        while y_index < contents.len() {
            let letter = contents[y_index].get(x_index..x_index+1).unwrap();

            if letter == "#" {
                num_trees += 1;
            }

            x_index = modulo(x_index + deltas[current_slope][0], contents[y_index].len());
            y_index += deltas[current_slope][1];
        }

        multiplied_trees = multiplied_trees * num_trees;
    }

    println!("{}", multiplied_trees);
}

fn modulo(a: usize, b: usize) -> usize {
    ((a % b) + b) % b
}