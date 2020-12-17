pub fn solve_part_1(input: &str) -> u32 {
    let mut lines = input.split("\n");

    let earliest_time: u32 =
        lines
            .next()
            .expect("Invalid input!")
            .parse()
            .expect("Invalid input!");

    let bus_ids: Vec<u32> =
        lines
            .next()
            .expect("Invalid input!")
            .split(",")
            .filter(|id| *id != "x")
            .map(|id| id.parse::<u32>().expect("Invalid input!"))
            .collect();

    let (first_bus_id, shortest_wait): (u32, u32) =
        bus_ids
            .iter()
            .map(|id| {
                let wait: u32 = id - (earliest_time % id);
                (*id, wait)
            })
            .min_by(|(_, wait1), (_, wait2)| wait1.cmp(wait2))
            .expect("No solution!");

    first_bus_id * shortest_wait
}

pub fn solve_part_2(input: &str) -> u64 {
    let mut lines = input.split("\n");
    let _ = lines.next();

    let mut indexed_bus_ids: Vec<(usize, u64)> =
        lines
            .next()
            .expect("Invalid input!")
            .split(",")
            .enumerate()
            .filter(|(_, id)| *id != "x")
            .map(|(index, id)| {
                let id = id.parse::<u64>().expect("Invalid input!");
                (index % id as usize, id)
            })
            .collect();

    indexed_bus_ids.sort_unstable_by(|(_, id1), (_, id2)| id1.cmp(id2));
    indexed_bus_ids.reverse();
    let indexed_bus_ids = indexed_bus_ids;

    println!("{:?}", indexed_bus_ids);

    // todo: fold (chinese remainder theorem, extended euclidean algorithm)

    0
}

#[cfg(test)]
mod tests {
    use crate::day13;

    #[test]
    fn given_example_1() {
        let input: &str = "939\n\
                           7,13,x,x,59,x,31,19";

        assert_eq!(day13::solve_part_1(&input), 295);
        assert_eq!(day13::solve_part_2(&input), 1068781);
    }

    #[test]
    fn given_example_2() {
        let input: &str = "\n\
                           17,x,13,19";

        assert_eq!(day13::solve_part_2(&input), 3417);
    }

    #[test]
    fn given_example_3() {
        let input: &str = "\n\
                           67,7,59,61";

        assert_eq!(day13::solve_part_2(&input), 754018);
    }

    #[test]
    fn given_example_4() {
        let input: &str = "\n\
                           67,x,7,59,61";

        assert_eq!(day13::solve_part_2(&input), 779210);
    }

    #[test]
    fn given_example_5() {
        let input: &str = "\n\
                           67,7,x,59,61";

        assert_eq!(day13::solve_part_2(&input), 1261476);
    }

    #[test]
    fn given_example_6() {
        let input: &str = "\n\
                           1789,37,47,1889";

        assert_eq!(day13::solve_part_2(&input), 1202161486);
    }
}
