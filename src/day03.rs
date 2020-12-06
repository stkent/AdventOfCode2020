pub fn solve_part_1(input: &Vec<String>) -> u32 {
    trees_encountered(input, 3, 1)
}

pub fn solve_part_2(input: &Vec<String>) -> u32 {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |acc, path| {
            acc * trees_encountered(input, path.0, path.1)
        })
}

pub fn trees_encountered(input: &Vec<String>, dx: u32, dy: u32) -> u32 {
    let width = input[0].len() as u32;

    input.iter()
        .step_by(dy as usize)
        .enumerate()
        .skip(1)
        .map(|(step, row)| {
            let x = ((step as u32) * dx) % width;
            row.as_bytes()[x as usize]
        })
        .filter(|object| *object == b'#')
        .count() as u32
}
