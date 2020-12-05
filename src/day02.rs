use regex::Regex;

pub fn solve_part_1(input: &Vec<String>) -> u32 {
    let re = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<char>\w): (?P<password>\w+)$").unwrap();

    input
        .into_iter()
        .map(|line| {
            let caps = re.captures(line).unwrap();

            StoredPassword {
                value: caps.get(4).unwrap().as_str().to_string(),
                policy: Policy {
                    character: caps.name("char").unwrap().as_str().parse().unwrap(),
                    min: caps.name("min").unwrap().as_str().parse().unwrap(),
                    max: caps.name("max").unwrap().as_str().parse().unwrap(),
                },
            }
        })
        .filter(|stored| stored.is_valid())
        .count() as u32
}

struct StoredPassword {
    value: String,
    policy: Policy,
}

struct Policy {
    character: char,
    min: u32,
    max: u32,
}

impl StoredPassword {
    fn is_valid(&self) -> bool {
        let count = self
            .value
            .chars()
            .filter(|char| *char == self.policy.character)
            .count() as u32;

        count >= self.policy.min && count <= self.policy.max
    }
}
