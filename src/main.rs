use std::fs::read_to_string;

mod day01;

fn main() {
    // Day 01
    let input_01: Vec<u32> = file_lines("./input01.txt")
        .into_iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    println!("Day  1, Part 1: {}", day01::solve_part_1(&input_01));
    println!("Day  1, Part 2: {}", day01::solve_part_2(&input_01));
}

fn file_lines(file_name: &str) -> Vec<String> {
    read_to_string(file_name)
        .expect("File not found!")
        .lines()
        .map(|line| line.to_string())
        .collect()
}
