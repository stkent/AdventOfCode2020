use std::ops::{RangeInclusive, Not};

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> u32 {
    let parts: Vec<&str> =
        input
            .split("\n\n")
            .collect();

    let rules: Vec<Rule> =
        parts[0]
            .split('\n')
            .map(Rule::from)
            .collect();

    let nearby: Vec<Ticket> =
        parts[2]
            .split('\n')
            .dropping(1)
            .map(Ticket::from)
            .collect();

    nearby
        .iter()
        .map(|ticket| -> u32 {
            let ticket_scan_error_rate =
                ticket
                    .fields
                    .iter()
                    .filter(|ticket_field| {
                        rules
                            .iter()
                            .any(|rule| rule.matches(**ticket_field))
                            .not()
                    })
                    .sum();

            ticket_scan_error_rate
        })
        .sum()
}

#[derive(Debug)]
struct Rule {
    valid_ranges: Vec<RangeInclusive<u32>>
}

fn parse_range(string: &str) -> RangeInclusive<u32> {
    let bounds: Vec<u32> =
        string
            .split('-')
            .map(|num: &str| num.parse::<u32>().expect("Invalid input!"))
            .collect();

    RangeInclusive::new(bounds[0], bounds[1])
}

impl Rule {
    fn from(line: &str) -> Rule {
        let ranges_start_index: usize =
            line
                .find(": ")
                .expect("Invalid input!");

        let valid_ranges: Vec<RangeInclusive<u32>> =
            line
                .split_at(ranges_start_index + 2)
                .1
                .split(" or ")
                .map(|range: &str| parse_range(range))
                .collect();

        Rule { valid_ranges }
    }
}

impl Rule {
    fn matches(&self, value: u32) -> bool {
        self.valid_ranges
            .iter()
            .any(|valid_range| valid_range.contains(&value))
    }
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<u32>
}

impl Ticket {
    fn from(_line: &str) -> Ticket {
        let fields: Vec<u32> =
            _line
                .split(',')
                .map(|entry| entry.parse().expect("Invalid input!"))
                .collect();

        Ticket { fields }
    }
}

#[cfg(test)]
mod tests {
    use crate::day16::solve_part_1;

    #[test]
    fn given_examples() {
        let input = "class: 1-3 or 5-7\n\
                     row: 6-11 or 33-44\n\
                     seat: 13-40 or 45-50\n\
                     \n\
                     your ticket:\n\
                     7,1,14\n\
                     \n\
                     nearby tickets:\n\
                     7,3,47\n\
                     40,4,50\n\
                     55,2,20\n\
                     38,6,12";

        assert_eq!(solve_part_1(input), 71);
    }
}
