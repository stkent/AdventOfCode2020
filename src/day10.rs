use itertools::Itertools;

pub fn solve_part_1(input: &[u8]) -> u16 {
    let mut input_copy: Vec<u8> = input.to_vec();
    input_copy.sort_unstable();

    let jumps: Vec<u8> =
        input_copy
            .iter()
            .zip(input_copy.iter().skip(1))
            .map(|(lo, hi)| hi - lo)
            .collect();

    let one_jumps: usize =
        jumps
            .iter()
            .filter(|&jump| *jump == 1)
            .count();

    let three_jumps: usize =
        jumps
            .iter()
            .filter(|&jump| *jump == 3)
            .count();

    ((one_jumps + 1) * (three_jumps + 1)) as u16
}

pub fn solve_part_2() -> u64 {
    // See input10_annotated.txt for reasoning!
    7 * 7 * 7 * 7 * 2 * 7 * 4 * 2 * 7 * 2 * 7 * 4 * 2 * 2 * 2 * 7 * 7 * 4 * 7 * 2 * 2 * 7
}

#[cfg(test)]
mod tests {
    use crate::day10;

    #[test]
    fn given_example_1() {
        let input: Vec<u8> =
            vec![
                16,
                10,
                15,
                5,
                1,
                11,
                7,
                19,
                6,
                12,
                4,
            ];

        assert_eq!(day10::solve_part_1(&input), 35);
    }

    #[test]
    fn given_example_2() {
        let input: Vec<u8> =
            vec![
                28,
                33,
                18,
                42,
                31,
                14,
                46,
                20,
                48,
                47,
                24,
                23,
                49,
                45,
                19,
                38,
                39,
                11,
                1,
                32,
                25,
                35,
                8,
                17,
                7,
                9,
                4,
                2,
                34,
                10,
                3,
            ];

        assert_eq!(day10::solve_part_1(&input), 220);
    }
}
