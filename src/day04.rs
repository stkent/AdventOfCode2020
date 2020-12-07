use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

pub fn solve_part_1(input: &str) -> u16 {
    complete_passports(input)
        .iter()
        .count() as u16
}

pub fn solve_part_2(input: &str) -> u16 {
    complete_passports(input)
        .iter()
        .filter(|passport| passport.is_valid())
        .count() as u16
}

fn complete_passports(input: &str) -> Vec<Passport> {
    input.split("\n\n")
        .filter_map(|lines| Passport::from(lines))
        .collect()
}

struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

impl Passport {
    fn from(s: &str) -> Option<Passport> {
        let mut raw_data: HashMap<String, String> = HashMap::new();

        s.split_ascii_whitespace()
            .into_iter()
            .for_each(|attr| {
                let (key, value) = (attr[..=2].to_string(), attr[4..].to_string());
                raw_data.insert(key, value);
            });

        let byr: u16 = match raw_data.get("byr").and_then(|s| s.parse().ok()) {
            Some(byr) => byr,
            None => return None
        };

        let iyr: u16 = match raw_data.get("iyr").and_then(|s| s.parse().ok()) {
            Some(iyr) => iyr,
            None => return None
        };

        let eyr: u16 = match raw_data.get("eyr").and_then(|s| s.parse().ok()) {
            Some(eyr) => eyr,
            None => return None
        };

        let hgt: String = match raw_data.get("hgt") {
            Some(hgt) => hgt.to_string(),
            None => return None
        };

        let hcl: String = match raw_data.get("hcl") {
            Some(hcl) => hcl.to_string(),
            None => return None
        };

        let ecl: String = match raw_data.get("ecl") {
            Some(ecl) => ecl.to_string(),
            None => return None
        };

        let pid: String = match raw_data.get("pid") {
            Some(pid) => pid.to_string(),
            None => return None
        };

        Some(Passport { byr, iyr, eyr, hgt, hcl, ecl, pid })
    }

    fn is_valid(&self) -> bool {
        let hgt_len = self.hgt.len();

        let hgt_value: u16 = match self.hgt[..hgt_len - 2].parse().ok() {
            Some(hgt) => hgt,
            None => return false
        };

        let hgt_valid: bool = if self.hgt.ends_with("cm") {
            hgt_value >= 150 && hgt_value <= 193
        } else if self.hgt.ends_with("in") {
            hgt_value >= 59 && hgt_value <= 76
        } else {
            return false;
        };

        self.byr >= 1920 && self.byr <= 2002 &&
            self.iyr >= 2010 && self.iyr <= 2020 &&
            self.eyr >= 2020 && self.eyr <= 2030 &&
            hgt_valid &&
            HCL_REGEX.is_match(&self.hcl) &&
            ECL_REGEX.is_match(&self.ecl) &&
            PID_REGEX.is_match(&self.pid)
    }
}

lazy_static! {
    static ref HCL_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref ECL_REGEX: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
}

#[cfg(test)]
mod tests {
    use crate::day04;

    #[test]
    fn invalid_examples() {
        let input = "eyr:1972 cid:100\n\
                     hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
                     \n\
                     iyr:2019\n\
                     hcl:#602927 eyr:1967 hgt:170cm\n\
                     ecl:grn pid:012533040 byr:1946\n\
                     \n\
                     hcl:dab227 iyr:2012\n\
                     ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
                     \n\
                     hgt:59cm ecl:zzz\n\
                     eyr:2038 hcl:74454a iyr:2023\n\
                     pid:3556412378 byr:2007";

        let valid = day04::solve_part_2(input);

        assert_eq!(valid, 0)
    }

    #[test]
    fn valid_examples() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
                     hcl:#623a2f\n\
                     \n\
                     eyr:2029 ecl:blu cid:129 byr:1989\n\
                     iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
                     \n\
                     hcl:#888785\n\
                     hgt:164cm byr:2001 iyr:2015 cid:88\n\
                     pid:545766238 ecl:hzl\n\
                     eyr:2022\n\
                     \n\
                     iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let valid = day04::solve_part_2(input);

        assert_eq!(valid, 4)
    }
}
