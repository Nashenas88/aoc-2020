use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashSet;

pub struct Group {
    set: Vec<Vec<char>>,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Group> {
    input
        .replace("\n", " ")
        .split("  ")
        .map(|line| Group {
            set: line
                .split(" ")
                .map(|chunk| chunk.chars().collect::<Vec<_>>())
                .filter(|v| v.len() > 0)
                .collect::<Vec<_>>(),
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Group]) -> usize {
    input
        .iter()
        .map(|g| g.set.iter().flatten().collect::<HashSet<_>>().len())
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Group]) -> usize {
    input
        .iter()
        .map(|g| -> usize {
            let set = g.set.iter().flatten().collect::<HashSet<_>>();
            set.iter()
                .map(|c| (g.set.len() > 0 && g.set.iter().all(|p| p.contains(c))) as usize)
                .sum()
        })
        .sum()
}

#[test]
fn test_input() {
    let contents = "abc

a
b
c

ab
ac

a
a
a
a

b
";

    let input = input_generator(contents);
    assert_eq!(input.len(), 5);
    let result = solve_part1(&input);
    assert_eq!(result, 11);
    let result = solve_part2(&input);
    assert_eq!(result, 6);
}
