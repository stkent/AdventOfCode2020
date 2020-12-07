use std::collections::HashSet;

pub fn solve_part_1(input: &str) -> u16 {
    input.split("\n\n")
        .map(|lines| {
            let mut ayes = HashSet::<char>::new();
            lines.split("\n")
                .into_iter()
                .for_each(|line| ayes.extend(line.chars().into_iter()));

            ayes
        })
        .map(|ayes| ayes.into_iter().count())
        .sum::<usize>() as u16
}

#[cfg(test)]
mod tests {
    use crate::day06;

    #[test]
    fn part_1_example() {
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

        let count_sum = day06::solve_part_1(input);

        assert_eq!(count_sum, 11)
    }

}