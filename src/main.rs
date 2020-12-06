use std::fs::read_to_string;

mod day01;
mod day02;
mod day03;
mod day05;

fn main() {
    // Day 01
    let input01: Vec<u32> = file_lines("./input01.txt")
        .into_iter()
        .map(|line| line.parse::<u32>().unwrap()).collect();

    println!("Day  1, Part 1: {}", day01::solve_part_1(&input01));
    println!("Day  1, Part 2: {}", day01::solve_part_2(&input01));

    // Day 02
    let input02: Vec<String> = file_lines("./input02.txt");
    println!("Day  2, Part 1: {}", day02::solve_part_1(&input02));
    println!("Day  2, Part 2: {}", day02::solve_part_2(&input02));

    // Day 03
    let input03: Vec<String> = file_lines("./input03.txt");
    println!("Day  3, Part 1: {}", day03::solve_part_1(&input03));
    println!("Day  3, Part 2: {}", day03::solve_part_2(&input03));

    // Day 03
    let input05: Vec<String> = file_lines("./input05.txt");
    println!("Day  5, Part 1: {}", day05::solve_part_1(&input05));
    println!("Day  5, Part 2: {}", day05::solve_part_2(&input05));
}

fn file_lines(file_name: &str) -> Vec<String> {
    read_to_string(file_name)
        .expect("File not found!")
        .lines()
        .map(|line| line.to_string())
        .collect()
}
