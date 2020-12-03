use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use regex::Regex;

pub struct PasswordPolicy {
    lower: usize,
    upper: usize,
    letter: char,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(PasswordPolicy, String)> {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]+): ([a-z]+)$").unwrap();
    input
        .lines()
        .map(|l| {
            let captures = re.captures_iter(l).next().unwrap();
            (
                PasswordPolicy {
                    lower: captures[1].parse().unwrap(),
                    upper: captures[2].parse().unwrap(),
                    letter: captures[3].chars().next().unwrap(),
                },
                captures[4].into(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(PasswordPolicy, String)]) -> u32 {
    input
        .iter()
        .map(|(policy, s)| {
            let count: usize = s.chars().map(|c| (c == policy.letter) as usize).sum();
            (!(count > policy.upper || count < policy.lower)) as u32
        })
        .sum()
}

fn char_at(s: &str, i: usize) -> char {
    s.chars().skip(i - 1).next().unwrap()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(PasswordPolicy, String)]) -> u32 {
    input
        .iter()
        .map(|(policy, s)| {
            ((char_at(s, policy.lower) == policy.letter)
                ^ (char_at(s, policy.upper) == policy.letter)) as u32
        })
        .sum()
}
