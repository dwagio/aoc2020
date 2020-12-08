use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read input file");
    let program: Vec<&str> = contents.lines().collect();

    let final_acc = execute_program(program);
    println!("{}", final_acc);
}

fn execute_program(program: Vec<&str>) -> isize {
    let mut acc: isize = 0;
    let mut instruction_ptr: isize = 0;

    let mut visited = HashSet::new();

    loop {
        if visited.contains(&instruction_ptr) {
            break;
        }

        visited.insert(instruction_ptr);

        let mut split_instruction = program[instruction_ptr as usize].split(" ");
        let instruction = split_instruction.next().unwrap();
        let arg = split_instruction.next().unwrap().parse::<isize>().unwrap();

        match instruction {
            "nop" => {
                instruction_ptr += 1;
            }
            "acc" => {
                acc += arg;
                instruction_ptr += 1;
            }
            "jmp" => {
                instruction_ptr += arg;
            }
            _ => {}
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_program() {
        let contents = fs::read_to_string("test_input").expect("Failed to read input file");
        let program: Vec<&str> = contents.lines().collect();

        let final_acc = execute_program(program);

        assert_eq!(final_acc, 5);
    }
}
