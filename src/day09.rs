use Ordering::{Equal, Greater, Less};
use std::cmp::Ordering;
use std::ops::Range;

pub fn solve_part_1(input: &[u64], window_size: usize) -> u64 {
    'outer: for target_index in window_size..input.len() {
        let target: u64 = input[target_index];

        let mut window: Vec<u64> = input[(target_index - window_size)..target_index].to_vec();
        window.sort();

        let mut bot_index: usize = 0;
        let mut top_index: usize = window_size - 1;

        while bot_index < top_index {
            match (window[bot_index] + window[top_index]).cmp(&target) {
                Less => { bot_index += 1 }
                Equal => continue 'outer,
                Greater => { top_index -= 1 }
            }
        }

        return target;
    }

    panic!("No solution!");
}

pub fn solve_part_2(input: &[u64], target: u64) -> u64 {
    'outer: for start_index in 0..(input.len() - 1) {
        for end_index in (start_index + 1)..input.len() {
            let window: &[u64] = &input[start_index..=end_index];
            let sum: u64 = window.iter().sum();

            match sum.cmp(&target) {
                Less => { /* Do nothing, expand window. */ }
                Equal => {
                    let max = window.iter().max().unwrap();
                    let min = window.iter().min().unwrap();
                    return min + max;
                }
                Greater => { continue 'outer; }
            }
        }
    }

    panic!("No solution!");
}

#[cfg(test)]
mod tests {
    use crate::day09;

    #[test]
    fn given_example() {
        let input: Vec<u64> =
            vec![
                35,
                20,
                15,
                25,
                47,
                40,
                62,
                55,
                65,
                95,
                102,
                117,
                150,
                182,
                127,
                219,
                299,
                277,
                309,
                576,
            ];

        assert_eq!(day09::solve_part_1(&input, 5), 127);
        assert_eq!(day09::solve_part_2(&input, 127), 62);
    }
}
