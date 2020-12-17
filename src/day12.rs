use crate::day12::Instruction::{East, North, South, West, Left, Right, Forward};
use enum_ordinalize::Ordinalize;

pub fn solve_part_1(input: &[String]) -> u32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut direction = Direction::East;

    input
        .iter()
        .map(|line| parse_instruction(line))
        .for_each(|(instruction, value)| match instruction {
            North => y += value as i32,
            East => x += value as i32,
            South => y -= value as i32,
            West => x -= value as i32,
            Left => {
                let right_turns = (4 - value / 90) % 4;
                let new_ordinal = (direction.ordinal() + right_turns as i8) % 4;
                direction = Direction::from_ordinal(new_ordinal).unwrap()
            }
            Right => {
                let right_turns = (value / 90) % 4;
                let new_ordinal = (direction.ordinal() + right_turns as i8) % 4;
                direction = Direction::from_ordinal(new_ordinal).unwrap()
            }
            Forward => match direction {
                Direction::North => y += value as i32,
                Direction::East => x += value as i32,
                Direction::South => y -= value as i32,
                Direction::West => x -= value as i32,
            },
        });

    return (x.abs() + y.abs()) as u32
}

pub fn solve_part_2(input: &[String]) -> u32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut waypoint_x: i32 = 10;
    let mut waypoint_y: i32 = 1;

    input
        .iter()
        .map(|line| parse_instruction(line))
        .for_each(|(instruction, value)| match instruction {
            North => waypoint_y += value as i32,
            East => waypoint_x += value as i32,
            South => waypoint_y -= value as i32,
            West => waypoint_x -= value as i32,
            Left => {
                let left_turns = (value / 90) % 4;
                match left_turns {
                    1 => {
                        let tmp = waypoint_x;
                        waypoint_x = -waypoint_y;
                        waypoint_y = tmp;
                    }

                    2 => {
                        waypoint_x = -waypoint_x;
                        waypoint_y = -waypoint_y;
                    }

                    3 => {
                        let tmp = waypoint_x;
                        waypoint_x = waypoint_y;
                        waypoint_y = -tmp;
                    }

                    _ => {}
                }
            }
            Right => {
                let right_turns = (value / 90) % 4;
                match right_turns {
                    1 => {
                        let tmp = waypoint_x;
                        waypoint_x = waypoint_y;
                        waypoint_y = -tmp;
                    }

                    2 => {
                        waypoint_x = -waypoint_x;
                        waypoint_y = -waypoint_y;
                    }

                    3 => {
                        let tmp = waypoint_x;
                        waypoint_x = -waypoint_y;
                        waypoint_y = tmp;
                    }

                    _ => {}
                }
            }
            Forward => {
                x += value as i32 * waypoint_x;
                y += value as i32 * waypoint_y;
            },
        });

    return (x.abs() + y.abs()) as u32
}

#[derive(Ordinalize)]
enum Direction {
    North,
    East,
    South,
    West,
}

enum Instruction {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}

fn parse_instruction(string: &str) -> (Instruction, u32) {
    let value: u32 = string[1..].parse().unwrap();

    let instruction: Instruction = match string.chars().next().unwrap() {
        'N' => North,
        'E' => East,
        'S' => South,
        'W' => West,
        'L' => Left,
        'R' => Right,
        'F' => Forward,
        _ => panic!("Invalid instruction!")
    };

    (instruction, value)
}

#[cfg(test)]
mod tests {
    use crate::day12;

    #[test]
    fn given_example_1() {
        let input: Vec<String> =
            vec![
                "F10",
                "N3",
                "F7",
                "R90",
                "F11",
            ]
                .iter()
                .map(ToString::to_string)
                .collect();

        assert_eq!(day12::solve_part_1(&input), 25);
        assert_eq!(day12::solve_part_2(&input), 286);
    }
}
