use std::collections::HashMap;

pub fn solve_part_1(input: &str) -> u8 {
    let passports: Vec<Passport> = input.split("\n\n")
        .map(|lines| build_passport(lines))
        .collect();

    println!("{}", passports.first().unwrap()["iyr"]);

    passports
        .into_iter()
        .filter(|passport| {
            passport.contains_key("byr") &&
                passport.contains_key("iyr") &&
                passport.contains_key("eyr") &&
                passport.contains_key("hgt") &&
                passport.contains_key("hcl") &&
                passport.contains_key("ecl") &&
                passport.contains_key("pid")
        })
        .count() as u8
}

type Passport = HashMap<String, String>;

fn build_passport(lines: &str) -> Passport {
    let mut result: HashMap<String, String> = HashMap::new();

    lines.split_ascii_whitespace()
        .into_iter()
        .for_each(|attr| {
            result.insert(
                attr[..=2].to_string(),
                attr[4..].to_string(),
            );
        });

    result
}
