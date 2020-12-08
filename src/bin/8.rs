extern crate aoc2020;

use aoc2020::utils;

fn parse_line(line: &str) -> (String, i32) {
    let mut parts = line.split(" ");
    let op = parts.next().unwrap();
    let value = parts.next().unwrap().parse::<i32>().unwrap();

    (op.to_string(), value)
}

fn run_program(instructions: &Vec<(String, i32)>) -> Result<i32, i32> {
    let mut executed_instructions: Vec<bool> = vec![false; instructions.len()];

    let mut acc = 0;
    let mut pc: usize = 0;
    loop {
        if pc == instructions.len() {
            // program finished!
            return Ok(acc);
        }

        let instruction = &instructions[pc];
        let op = &instruction.0;
        let value = instruction.1;

        if executed_instructions[pc] {
            // Executing the same instruction for a second time, we're in a cycle
            return Err(acc);
        }

        executed_instructions[pc] = true;

        if op == "jmp" {
            pc = (pc as i32 + value) as usize;
        } else {
            pc += 1;
        }

        if op == "acc" {
            acc += value;
        }
    }
}

fn main() {
    let mut instructions: Vec<(String, i32)> = utils::read_lines("./input_data/8.txt")
        .iter().map(|l| parse_line(l)).collect();

    let part1 = run_program(&instructions);
    println!("Part 1: {}", part1.unwrap_err());

    for i in 0..instructions.len() {
        let swapped_op = instructions[i].0.clone();

        if swapped_op == "jmp" {
            instructions[i].0 = String::from("nop")
        } else if swapped_op == "nop" {
            instructions[i].0 = String::from("jmp")
        } else {
            // skip acc ops
            continue;
        }

        let result = run_program(&instructions);
        if result.is_ok() {
            println!("Part 2: {}", result.unwrap());
        }

        instructions[i].0 = swapped_op.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(parse_line("jmp -150"), ("jmp", -150));
        assert_eq!(parse_line("acc +41"), ("acc", 41));
    }
}
