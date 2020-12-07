use std::collections::HashSet;

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> u16 {
    count(input, crate::day06::part_1_group_scorer)
}

pub fn solve_part_2(input: &str) -> u16 {
    count(input, crate::day06::part_2_group_scorer)
}

fn count(input: &str, group_scorer: fn(&str) -> u16) -> u16 {
    input
        .split("\n\n")
        .map(|lines| group_scorer(lines))
        .sum()
}

fn part_1_group_scorer(group: &str) -> u16 {
    group
        .replace("\n", "")
        .chars()
        .unique()
        .count() as u16
}

fn part_2_group_scorer(group: &str) -> u16 {
    group
        .split("\n")
        .into_iter()
        .fold(
            "abcdefghijklmnopqrstuvwxyz".chars().collect::<HashSet<char>>(),
            |acc, line| {
                let person_ayes = line.chars().collect();
                acc.intersection(&person_ayes).cloned().collect()
            },
        )
        .len() as u16
}

#[cfg(test)]
mod tests {
    use crate::day06;

    #[test]
    fn given_example() {
        let input = "abc\n\
                     \n\
                     a\n\
                     b\n\
                     c\n\
                     \n\
                     ab\n\
                     ac\n\
                     \n\
                     a\n\
                     a\n\
                     a\n\
                     a\n\
                     \n\
                     b";

        assert_eq!(day06::solve_part_1(input), 11);
        assert_eq!(day06::solve_part_2(input), 6);
    }
}
