use regex::Regex;

pub fn solve_part_1(input: &Vec<String>) -> u32 {
    let re = Regex::new(r"^(?P<int1>\d+)-(?P<int2>\d+) (?P<char>\w): (?P<password>\w+)$").unwrap();

    input
        .into_iter()
        .map(|line| {
            let caps = re.captures(line).unwrap();

            StoredPassword {
                value: caps.name("password").unwrap().as_str().to_string(),
                policy: Policy {
                    character: caps.name("char").unwrap().as_str().parse().unwrap(),
                    int1: caps.name("int1").unwrap().as_str().parse().unwrap(),
                    int2: caps.name("int2").unwrap().as_str().parse().unwrap(),
                },
            }
        })
        .filter(|stored| stored.is_valid_part_1())
        .count() as u32
}

pub fn solve_part_2(input: &Vec<String>) -> u32 {
    let re = Regex::new(r"^(?P<int1>\d+)-(?P<int2>\d+) (?P<char>\w): (?P<password>\w+)$").unwrap();

    input
        .into_iter()
        .map(|line| {
            let caps = re.captures(line).unwrap();

            StoredPassword {
                value: caps.name("password").unwrap().as_str().to_string(),
                policy: Policy {
                    character: caps.name("char").unwrap().as_str().parse().unwrap(),
                    int1: caps.name("int1").unwrap().as_str().parse().unwrap(),
                    int2: caps.name("int2").unwrap().as_str().parse().unwrap(),
                },
            }
        })
        .filter(|stored| stored.is_valid_part_2())
        .count() as u32
}

struct StoredPassword {
    value: String,
    policy: Policy,
}

struct Policy {
    character: char,
    int1: u32,
    int2: u32,
}

impl StoredPassword {
    fn is_valid_part_1(&self) -> bool {
        let min_count = self.policy.int1;
        let max_count = self.policy.int2;

        let count = self
            .value
            .chars()
            .filter(|char| *char == self.policy.character)
            .count() as u32;

        count >= min_count && count <= max_count
    }
}

impl StoredPassword {
    fn is_valid_part_2(&self) -> bool {
        let index1 = self.policy.int1 - 1;
        let index2 = self.policy.int2 - 1;
        let char1 = self.value.as_bytes()[index1 as usize] as char;
        let char2 = self.value.as_bytes()[index2 as usize] as char;

        (char1 == self.policy.character || char2 == self.policy.character) && char1 != char2
    }
}
