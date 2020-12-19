pub fn solve_part_1(input: &[String]) -> u16 {
    let mut current_map: Vec<Vec<char>> =
        input
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

    loop {
        // println!("-------------");
        // print_map(&current_map);
        // println!("-------------");

        let new_map: &mut Vec<Vec<char>> =
            &mut vec![vec![FLOOR; current_map[0].len()]; current_map.len()];

        for x in 0usize..current_map[0].len() {
            for y in 0usize..current_map.len() {
                let current: &char = &current_map[y][x];

                new_map[y][x] = match *current {
                    FLOOR => FLOOR,

                    EMPTY_SEAT => {
                        let neighbors: Vec<&char> = current_neighbors(x, y, &current_map);

                        let no_occupied_neighbors =
                            neighbors
                                .into_iter()
                                .all(|&neighbor| neighbor != OCCUPIED_SEAT);

                        if no_occupied_neighbors { OCCUPIED_SEAT } else { EMPTY_SEAT }
                    }

                    OCCUPIED_SEAT => {
                        let neighbors: Vec<&char> = current_neighbors(x, y, &current_map);

                        let occupied_neighbor_count =
                            neighbors
                                .into_iter()
                                .filter(|&&neighbor| neighbor == OCCUPIED_SEAT)
                                .count();

                        if occupied_neighbor_count >= 4 { EMPTY_SEAT } else { OCCUPIED_SEAT }
                    }

                    _ => panic!("Invalid byte!")
                };
            }
        }

        let mut did_change = false;

        'outer: for x in 0usize..current_map[0].len() {
            for y in 0usize..current_map.len() {
                did_change = did_change || current_map[y][x] != new_map[y][x]
            }
        }

        if !did_change {
            break;
        } else {
            current_map = new_map.to_owned();
        }
    }

    let mut result: u16 = 0;

    for x in 0usize..current_map[0].len() {
        for y in 0usize..current_map.len() {
            if current_map[y][x] == OCCUPIED_SEAT {
                result += 1;
            }
        }
    }

    result
}

fn current_neighbors(x: usize, y: usize, current_map: &Vec<Vec<char>>) -> Vec<&char> {
    let mut result: Vec<&char> = vec![];

    for dx in -1isize..=1 {
        for dy in -1isize..=1 {
            if dx == 0 && dy == 0 { continue; }

            let nx: isize = (x as isize) + dx;
            let ny: isize = (y as isize) + dy;

            if nx < 0 { continue; }
            if ny < 0 { continue; }

            let nx = nx as usize;
            let ny = ny as usize;
            if nx >= current_map[0].len() { continue; }
            if ny >= current_map.len() { continue; }

            result.push(&current_map[ny][nx]);
        }
    }

    result
}

const FLOOR: char = '.';
const EMPTY_SEAT: char = 'L';
const OCCUPIED_SEAT: char = '#';

fn print_map(map: &Vec<Vec<char>>) {
    for y in 0usize..map.len() {
        for x in 0usize..map[0].len() {
            print!("{}", map[y][x]);
        }

        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use crate::day11;

    #[test]
    fn given_example() {
        let input: Vec<String> =
            vec![
                "L.LL.LL.LL",
                "LLLLLLL.LL",
                "L.L.L..L..",
                "LLLL.LL.LL",
                "L.LL.LL.LL",
                "L.LLLLL.LL",
                "..L.L.....",
                "LLLLLLLLLL",
                "L.LLLLLL.L",
                "L.LLLLL.LL",
            ]
                .iter()
                .map(ToString::to_string)
                .collect();

        assert_eq!(day11::solve_part_1(&input), 37);
    }
}
