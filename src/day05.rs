pub fn solve_part_1(input: &Vec<String>) -> u16 {
    seat_ids(input)
        .into_iter()
        .max()
        .expect("No solution!")
}

pub fn solve_part_2(input: &Vec<String>) -> u16 {
    let mut ids: Vec<u16> = seat_ids(input);
    ids.sort_unstable();

    let (id_before_own, _) = ids
        .iter()
        .zip(ids.iter().skip(1))
        .find(|(id1, id2)| *id2 - *id1 > 1)
        .expect("No solution!");

    id_before_own + 1
}

fn seat_ids(input: &Vec<String>) -> Vec<u16> {
    input
        .into_iter()
        .map(|input_line| BoardingPass::from(input_line))
        .map(|pass| pass.seat_id())
        .collect()
}

struct BoardingPass {
    row: u8,
    col: u8,
}

impl BoardingPass {
    fn from(s: &str) -> BoardingPass {
        let row = s[..=6]
            .replace("F", "0")
            .replace("B", "1");

        let row = u8::from_str_radix(&row, 2).expect("Invalid input!");

        let column = s[7..]
            .replace("L", "0")
            .replace("R", "1");

        let col = u8::from_str_radix(&column, 2).expect("Invalid input!");

        BoardingPass { row, col }
    }

    fn seat_id(&self) -> u16 {
        u16::from(self.row) * 8 + u16::from(self.col)
    }
}
