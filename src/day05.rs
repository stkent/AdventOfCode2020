use regex::Regex;

pub fn solve_part_1(input: &Vec<String>) -> u16 {
    let re = Regex::new(r"^(?P<binary_row>[FB]{7})(?P<binary_column>[RL]{3})$").unwrap();

    input
        .into_iter()
        .map(|line| {
            let caps = re.captures(line).unwrap();

            let row = caps.name("binary_row")
                .unwrap()
                .as_str()
                .as_bytes()
                .iter()
                .enumerate()
                .fold(0, |acc, (index, &char)| {
                    let digit = match char {
                        b'F' => 0,
                        b'B' => 1,
                        _ => panic!("Invalid input!")
                    };

                    acc + (digit << (6 - index))
                });

            let column = caps.name("binary_column")
                .unwrap()
                .as_str()
                .as_bytes()
                .iter()
                .enumerate()
                .fold(0, |acc, (index, &char)| {
                    let digit = match char {
                        b'L' => 0,
                        b'R' => 1,
                        _ => panic!("Invalid input!")
                    };

                    acc + (digit << (2 - index))
                });

            BoardingPass {
                row,
                column,
            }
        })
        .map(|boarding_pass| boarding_pass.seat_id())
        .max()
        .unwrap()
}

struct BoardingPass {
    row: u8,
    column: u8,
}

impl BoardingPass {
    fn seat_id(&self) -> u16 {
        u16::from(self.row) * 8 + u16::from(self.column)
    }
}
