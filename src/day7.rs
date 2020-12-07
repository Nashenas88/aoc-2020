use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct Index {
    count: usize,
    location: usize,
}

pub struct Bag {
    name: String,
    holds: Vec<Index>,
}

impl Bag {
    fn contains_other(&self, bags: &Vec<Bag>, needle: &str) -> bool {
        if self
            .holds
            .iter()
            .any(|&Index { location, .. }| bags[location].name == needle)
        {
            return true;
        }

        self.holds
            .iter()
            .any(|&Index { location, .. }| bags[location].contains_other(bags, needle))
    }

    fn number_held(&self, bags: &Vec<Bag>) -> usize {
        self.holds
            .iter()
            .map(|&Index { count, location }| count + count * bags[location].number_held(bags))
            .sum::<usize>()
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Bag> {
    let bag_re = Regex::new(r"^([0-9]+) ([a-z]+ [a-z]+) bags?$").unwrap();
    let mut lookup_map = HashMap::new();
    let mut bags = vec![];
    for line in input.lines() {
        let mut parts = line.split(" bags contain ");
        let holder = String::from(parts.next().unwrap());
        let rest = parts.next().unwrap();
        assert!(parts.next().is_none());
        let rest = rest.trim_end_matches('.');
        let mut holds = vec![];
        for part in rest.split(", ") {
            let captures = if let Some(captures) = bag_re.captures(part) {
                captures
            } else {
                break; // no more matches
            };

            if let Some(val) = captures.get(1) {
                let held = String::from(&captures[2]);
                let location = lookup_map
                    .entry(held.clone())
                    .or_insert_with(|| {
                        bags.push(Bag {
                            name: held,
                            holds: vec![],
                        });
                        bags.len() - 1
                    })
                    .clone();
                holds.push(Index {
                    count: val.as_str().parse().unwrap(),
                    location,
                });
            }
        }

        match lookup_map.entry(holder.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(bags.len());
                bags.push(Bag { name: holder, holds });
            },
            Entry::Occupied(entry) => {
                bags[*entry.get()].holds = holds;
            }
        }
    }

    bags
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<Bag>) -> usize {
    input
        .iter()
        .filter(|bag| bag.name != "shiny gold")
        .map(|bag| bag.contains_other(input, "shiny gold") as usize)
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<Bag>) -> usize {
    input
        .iter()
        .find(|bag| bag.name == "shiny gold")
        .map(|bag| bag.number_held(input))
        .unwrap_or(0)
}

#[test]
fn test_input1() {
    let content = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 4);
}

#[test]
fn test_input2() {
    let content = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 32);
}

#[test]
fn test_input3() {
    let content = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 126);
}
