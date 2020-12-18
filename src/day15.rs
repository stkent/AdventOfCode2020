pub fn solve_part_1(input: &[u32]) -> u32 {
    let history: &mut Vec<u32> = &mut vec![];

    for index in 0..2020 {
        let spoken_number = number_to_speak(index, history, input);
        history.push(spoken_number);
    }

    *history.last().unwrap()
}

fn number_to_speak(index: usize, history: &[u32], seed: &[u32]) -> u32 {
    if index < seed.len() { return seed[index]; }

    let last_spoken_number: u32 =
        *history
            .last()
            .unwrap();

    let mut previously_spoken_index: Option<usize> = None;

    for previous_index in (0..index - 1).rev() {
        if history[previous_index] == last_spoken_number {
            previously_spoken_index = Some(previous_index);
            break;
        }
    }

    return match previously_spoken_index {
        Some(previous_index) => (index - 1 - previous_index) as u32,
        None => 0,
    };
}

#[cfg(test)]
mod tests {
    use crate::day15;

    #[test]
    fn given_examples() {
        assert_eq!(day15::solve_part_1(&vec![0, 3, 6]), 436);
        assert_eq!(day15::solve_part_1(&vec![1, 3, 2]), 1);
        assert_eq!(day15::solve_part_1(&vec![2, 1, 3]), 10);
        assert_eq!(day15::solve_part_1(&vec![1, 2, 3]), 27);
        assert_eq!(day15::solve_part_1(&vec![2, 3, 1]), 78);
        assert_eq!(day15::solve_part_1(&vec![3, 2, 1]), 438);
        assert_eq!(day15::solve_part_1(&vec![3, 1, 2]), 1836);
    }
}
