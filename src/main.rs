use std::fs::read_to_string;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day12;
mod day13;
mod day15;

fn main() {
    // Day 01
    let input01: Vec<u32> = file_strings("./input01.txt")
        .iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

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

    // Day 07
    let input07: Vec<String> = file_strings("./input07.txt");
    println!("Day  7, Part 1: {}", day07::solve_part_1(&input07));
    println!("Day  7, Part 2: {}", day07::solve_part_2(&input07));

    // Day 08
    let input08: Vec<String> = file_strings("./input08.txt");
    println!("Day  8, Part 1: {}", day08::solve_part_1(&input08));
    println!("Day  8, Part 2: {}", day08::solve_part_2(&input08));

    // Day 09
    let input09: Vec<u64> = file_strings("./input09.txt")
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    let day09_part1_result = day09::solve_part_1(&input09, 25);
    println!("Day  9, Part 1: {}", day09_part1_result);
    println!("Day  9, Part 2: {}", day09::solve_part_2(&input09, day09_part1_result));

    // Day 10
    let input10: Vec<u8> = file_strings("./input10.txt")
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    println!("Day 10, Part 1: {}", day10::solve_part_1(&input10));
    println!("Day 10, Part 2: {}", day10::solve_part_2());

    // Day 12
    let input12: Vec<String> = file_strings("./input12.txt");
    println!("Day 12, Part 1: {}", day12::solve_part_1(&input12));
    println!("Day 12, Part 2: {}", day12::solve_part_2(&input12));

    // Day 13
    let input13: String = file_string("./input13.txt");
    println!("Day 13, Part 1: {}", day13::solve_part_1(&input13));
    println!("Day 13, Part 2: {}", day13::solve_part_2(&input13));

    // Day 15
    let input15: Vec<u32> = vec![2, 0, 6, 12, 1, 3];
    println!("Day 15, Part 1: {}", day15::solve_part_1(&input15));
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
