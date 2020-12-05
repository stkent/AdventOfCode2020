pub fn solve_part_1(input: &Vec<u32>) -> u32 {
    for i in input {
        for j in input {
            if i + j == 2020 {
                return i * j;
            }
        }
    }

    panic!("No solution!")
}

pub fn solve_part_2(input: &Vec<u32>) -> u32 {
    for i in input {
        for j in input {
            for k in input {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }

    panic!("No solution!")
}
