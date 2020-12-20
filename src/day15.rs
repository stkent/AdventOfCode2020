use std::collections::HashMap;

pub fn solve_part_1(input: &[u32]) -> u32 {
    solve(input, 2020)
}

pub fn solve_part_2(input: &[u32]) -> u32 {
    solve(input, 30_000_000)
}

fn solve(input: &[u32], count: usize) -> u32 {
    let mut last: Option<u32> = None;
    let mut previous: HashMap<u32, usize> = HashMap::new();

    for index in 0..count {
        let next = compute_next(index, input, &last, &previous);

        if let Some(last) = last {
            previous.insert(last, index - 1);
        }

        last = Some(next);
    }

    last.unwrap()
}

fn compute_next(
    index: usize,
    start: &[u32],
    last: &Option<u32>,
    previous: &HashMap<u32, usize>,
) -> u32 {
    if index < start.len() {
        return start[index];
    }

    return match previous.get(&last.unwrap()) {
        Some(previous_index) => (index - 1 - previous_index) as u32,
        None => 0,
    };
}

#[cfg(test)]
mod tests {
    // use crate::day15;
    //
    // #[test]
    // fn given_examples() {
    //     assert_eq!(day15::solve_part_1(&vec![0, 3, 6]), 436);
    //     assert_eq!(day15::solve_part_1(&vec![1, 3, 2]), 1);
    //     assert_eq!(day15::solve_part_1(&vec![2, 1, 3]), 10);
    //     assert_eq!(day15::solve_part_1(&vec![1, 2, 3]), 27);
    //     assert_eq!(day15::solve_part_1(&vec![2, 3, 1]), 78);
    //     assert_eq!(day15::solve_part_1(&vec![3, 2, 1]), 438);
    //     assert_eq!(day15::solve_part_1(&vec![3, 1, 2]), 1836);
    //
    //     assert_eq!(day15::solve_part_2(&vec![0, 3, 6]), 175594);
    //     assert_eq!(day15::solve_part_2(&vec![1, 3, 2]), 2578);
    //     assert_eq!(day15::solve_part_2(&vec![2, 1, 3]), 3544142);
    //     assert_eq!(day15::solve_part_2(&vec![1, 2, 3]), 261214);
    //     assert_eq!(day15::solve_part_2(&vec![2, 3, 1]), 6895259);
    //     assert_eq!(day15::solve_part_2(&vec![3, 2, 1]), 18);
    //     assert_eq!(day15::solve_part_2(&vec![3, 1, 2]), 362);
    // }
}
