use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read input file");
    let program: Vec<&str> = contents.lines().collect();

    let final_acc = execute_program(program);
    println!("{}", final_acc);
}

fn execute_program(program: Vec<&str>) -> isize {
    let mut nops_and_jmps: Vec<isize> = vec![];

    for (i, line) in program.iter().enumerate() {
        let mut split_instruction = line.split(" ");
        let instruction = split_instruction.next().unwrap();

        if instruction == "nop" || instruction == "jmp" {
            nops_and_jmps.push(i as isize);
        }
    }

    let mut to_flip_iter = nops_and_jmps.iter();

    let mut finished = false;

    let mut acc: isize = 0;

    while !finished {
        let instr_to_flip = to_flip_iter.next().unwrap();
        acc = 0;
        let mut instruction_ptr: isize = 0;
        let mut visited = HashSet::new();
        finished = true;

        while instruction_ptr < program.len() as isize {
            if visited.contains(&instruction_ptr) {
                finished = false;
                break;
            }

            visited.insert(instruction_ptr);

            let mut split_instruction = program[instruction_ptr as usize].split(" ");
            let mut instruction = split_instruction.next().unwrap();
            let arg = split_instruction.next().unwrap().parse::<isize>().unwrap();

            if instruction_ptr == *instr_to_flip {
                instruction = match instruction {
                    "nop" => "jmp",
                    "jmp" => "nop",
                    _ => instruction,
                }
            }

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

        assert_eq!(final_acc, 8);
    }
}
