use std::fs::read_to_string;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    // Day 01
    let input01: Vec<u32> = file_strings("./input01.txt")
        .iter()
        .map(|line| line.parse::<u32>().unwrap()).collect();

    println!("Day  1, Part 1: {}", day01::solve_part_1(&input01));
    println!("Day  1, Part 2: {}", day01::solve_part_2(&input01));

    // Day 02
    let input02: Vec<String> = file_strings("./input02.txt");
    println!("Day  2, Part 1: {}", day02::solve_part_1(&input02));
    println!("Day  2, Part 2: {}", day02::solve_part_2(&input02));

    // Day 03
    let input03: Vec<String> = file_strings("./input03.txt");
    println!("Day  3, Part 1: {}", day03::solve_part_1(&input03));
    println!("Day  3, Part 2: {}", day03::solve_part_2(&input03));

    // Day 04
    let input04: String = file_string("./input04.txt");
    println!("Day  4, Part 1: {}", day04::solve_part_1(&input04));
    println!("Day  4, Part 2: {}", day04::solve_part_2(&input04));

    // Day 05
    let input05: Vec<String> = file_strings("./input05.txt");
    println!("Day  5, Part 1: {}", day05::solve_part_1(&input05));
    println!("Day  5, Part 2: {}", day05::solve_part_2(&input05));

    // Day 06
    let input06: String = file_string("./input06.txt");
    println!("Day  6, Part 1: {}", day06::solve_part_1(&input06));
    println!("Day  6, Part 2: {}", day06::solve_part_2(&input06));
}

fn file_strings(file_name: &str) -> Vec<String> {
    file_string(file_name)
        .lines()
        .map(ToString::to_string)
        .collect()
}

fn file_string(file_name: &str) -> String {
    read_to_string(file_name).expect("File not found!")
}
