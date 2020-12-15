use std::collections::HashMap;

use regex::Regex;

pub fn solve_part_1(input: &Vec<String>) -> u16 {
    let rules = parse_rules(input);
    rules
        .keys()
        .filter(|color| can_contain_other_color(color, GOLD, &rules))
        .count() as u16
}

pub fn solve_part_2(input: &Vec<String>) -> u16 {
    let rules = parse_rules(input);
    contained_bags(GOLD, &rules)
}

type Rules = HashMap<String, HashMap<String, u16>>;

const GOLD: &str = "shiny gold";

fn parse_rules(string: &Vec<String>) -> Rules {
    string
        .iter()
        .map(|rule| parse_rule(rule))
        .collect()
}

fn parse_rule(string: &str) -> (String, HashMap<String, u16>) {
    let rule_re =
        Regex::new(r"(?P<color>[\w ]+) bags contain (?P<inners>[\d\w, ]+)\.$")
            .unwrap();

    let inner_re =
        Regex::new(r"(?P<count>[\d]+) (?P<color>[\w ]+) bags?$")
            .unwrap();

    let rule_caps =
        rule_re
            .captures(string)
            .expect("Invalid input!");

    let outer_color =
        rule_caps
            .name("color")
            .unwrap()
            .as_str()
            .to_string();

    let inner_color_count_str =
        rule_caps
            .name("inners")
            .unwrap()
            .as_str();

    let inner_color_counts: HashMap<String, u16> =
        if inner_color_count_str.ends_with("no other bags") {
            HashMap::new()
        } else {
            inner_color_count_str
                .split(", ")
                .map(|inner| {
                    let inner_caps =
                        inner_re
                            .captures(inner)
                            .expect("Invalid input!");

                    let inner_color =
                        inner_caps
                            .name("color")
                            .expect("Invalid input!")
                            .as_str()
                            .to_string();

                    let inner_count: u16 =
                        inner_caps
                            .name("count")
                            .expect("Invalid input!")
                            .as_str()
                            .parse()
                            .unwrap();

                    (inner_color, inner_count)
                })
                .collect()
        };

    (outer_color, inner_color_counts)
}

fn can_contain_other_color(color: &str, other_color: &str, rules: &Rules) -> bool {
    let inner_colors: Vec<&str> =
        rules
            .get(color)
            .unwrap()
            .keys()
            .map(|color| color.as_ref())
            .collect();

    let directly_contains = inner_colors.contains(&GOLD);

    let indirectly_contains =
        inner_colors
            .iter()
            .any(|inner_color| can_contain_other_color(inner_color, other_color, rules));

    directly_contains || indirectly_contains
}

fn contained_bags(color: &str, rules: &Rules) -> u16 {
    let inner_color_counts: &HashMap<String, u16> =
        rules
            .get(color)
            .unwrap();

    let direct_count =
        inner_color_counts
            .values()
            .sum::<u16>();

    let indirect_count =
        inner_color_counts
            .iter()
            .map(|(inner_color, count)| count * contained_bags(inner_color, rules))
            .sum::<u16>();

    direct_count + indirect_count
}

#[cfg(test)]
mod tests {
    use crate::day07;

    #[test]
    fn given_example_1() {
        let input: Vec<String> =
            vec![
                "light red bags contain 1 bright white bag, 2 muted yellow bags.",
                "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
                "bright white bags contain 1 shiny gold bag.",
                "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
                "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
                "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
                "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
                "faded blue bags contain no other bags.",
                "dotted black bags contain no other bags."
            ]
                .iter()
                .map(ToString::to_string)
                .collect();

        assert_eq!(day07::solve_part_1(&input), 4);
        assert_eq!(day07::solve_part_2(&input), 32);
    }

    #[test]
    fn given_example_2() {
        let input: Vec<String> =
            vec![
                "shiny gold bags contain 2 dark red bags.",
                "dark red bags contain 2 dark orange bags.",
                "dark orange bags contain 2 dark yellow bags.",
                "dark yellow bags contain 2 dark green bags.",
                "dark green bags contain 2 dark blue bags.",
                "dark blue bags contain 2 dark violet bags.",
                "dark violet bags contain no other bags.",
            ]
                .iter()
                .map(ToString::to_string)
                .collect();

        assert_eq!(day07::solve_part_2(&input), 126);
    }
}
