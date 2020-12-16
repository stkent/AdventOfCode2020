use std::collections::HashSet;

use Instruction::{Acc, Jmp, Nop};
use TerminationType::{Never, Normal};

pub fn solve_part_1(input: &Vec<String>) -> isize {
    let instructions = parse_instructions(input);

    match execute(&instructions) {
        Never { acc } => acc,
        Normal { acc: _acc } => panic!("Unexpected result!"),
    }
}

pub fn solve_part_2(input: &Vec<String>) -> isize {
    let instructions = parse_instructions(input);

    for i in 0..instructions.len() {
        let instruction = instructions.get(i).unwrap();

        let substitute: Instruction = match instruction {
            Acc(value) => Acc(*value),
            Jmp(value) => Nop(*value),
            Nop(value) => Jmp(*value),
        };

        let mut modified_instructions = instructions.clone();
        modified_instructions[i] = substitute;

        match execute(&modified_instructions) {
            Never { acc: _acc } => continue,
            Normal { acc } => return acc
        }
    }

    panic!("Invalid input!")
}

fn parse_instructions(input: &Vec<String>) -> Vec<Instruction> {
    input
        .iter()
        .map(|line| Instruction::from(line))
        .collect()
}

fn execute(input: &Vec<Instruction>) -> TerminationType {
    let mut acc: isize = 0;
    let mut index: isize = 0;
    let mut executed: HashSet<usize> = HashSet::new();

    loop {
        let loop_detected = !executed.insert(index as usize);
        if loop_detected { return Never { acc }; }

        if (index as usize) == input.len() { return Normal { acc }; }

        let instruction =
            input
                .get(index as usize)
                .expect("Invalid index!");

        match instruction {
            Instruction::Acc(value) => {
                acc += value;
                index += 1;
            }

            Instruction::Jmp(value) => { index += value; }
            Instruction::Nop(_) => { index += 1; }
        }
    }
}

#[derive(Clone)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Instruction {
    fn from(s: &str) -> Instruction {
        let cmd: &str = &s[..3];

        let val: isize =
            (&s[4..])
                .parse()
                .expect("Invalid instruction!");

        match cmd {
            "acc" => Acc(val),
            "jmp" => Jmp(val),
            "nop" => Nop(val),
            _ => panic!("Invalid instruction!")
        }
    }
}

enum TerminationType {
    Never { acc: isize },
    Normal { acc: isize },
}

#[cfg(test)]
mod tests {
    use crate::day08;

    #[test]
    fn given_example() {
        let input: Vec<String> =
            vec![
                "nop +0",
                "acc +1",
                "jmp +4",
                "acc +3",
                "jmp -3",
                "acc -99",
                "acc +1",
                "jmp -4",
                "acc +6"
            ]
                .iter()
                .map(ToString::to_string)
                .collect();

        assert_eq!(day08::solve_part_1(&input), 5);
        assert_eq!(day08::solve_part_2(&input), 8);
    }
}
