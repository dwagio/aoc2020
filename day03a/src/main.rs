use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read input file!");

    let contents = contents.lines().collect::<Vec<&str>>();

    let mut num_trees = 0;
    let mut x_index = 0;
    let mut y_index = 0;

    while y_index < contents.len() {
        let letter = contents[y_index].get(x_index..x_index+1).unwrap();

        if letter == "#" {
            num_trees += 1;
        }

        x_index = modulo(x_index + 3, contents[y_index].len());
        y_index += 1;
    }

    println!("{}", num_trees);
}

fn modulo(a: usize, b: usize) -> usize {
    ((a % b) + b) % b
}