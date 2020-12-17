use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;
use std::ops::RangeInclusive;

pub struct Rule {
    name: String,
    overall: RangeInclusive<usize>,
    sections: Vec<RangeInclusive<usize>>,
}

impl Rule {
    pub fn new(name: String, initial: RangeInclusive<usize>) -> Self {
        Self {
            name,
            overall: initial.clone(),
            sections: vec![initial],
        }
    }

    pub fn add(&mut self, range: RangeInclusive<usize>) {
        if range.start() < self.overall.start() {
            self.overall = *range.start()..=*self.overall.end();
        }

        if range.end() > self.overall.end() {
            self.overall = *self.overall.start()..=*range.end();
        }

        self.sections.push(range);
    }

    pub fn contains(&self, value: usize) -> bool {
        if !self.overall.contains(&value) {
            return false;
        }

        for section in &self.sections {
            if section.contains(&value) {
                return true;
            }
        }

        false
    }
}

pub struct Ticket(Vec<usize>);

pub struct Input {
    rules: Vec<Rule>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    fn parse_ticket(line: &str) -> Ticket {
        Ticket(
            line.split(',')
                .map(|chunk| chunk.parse::<usize>().unwrap())
                .collect(),
        )
    }

    let mut lines = input.lines();
    let mut rules = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        rules.push({
            let mut split = line.split(": ");
            let name = String::from(split.next().unwrap());
            let mut ranges = split.next().unwrap().split(" or ").map(|chunk| {
                let mut parts = chunk.split('-').map(|i| i.parse::<usize>().unwrap());
                parts.next().unwrap()..=parts.next().unwrap()
            });
            let mut rule = Rule::new(name, ranges.next().unwrap());
            for range in ranges {
                rule.add(range);
            }

            rule
        });
    }
    assert_eq!(lines.next().unwrap(), "your ticket:");
    let your_ticket = parse_ticket(lines.next().unwrap());
    assert_eq!(lines.next().unwrap(), "");
    assert_eq!(lines.next().unwrap(), "nearby tickets:");
    let nearby_tickets = lines.map(parse_ticket).collect();
    Input {
        rules,
        your_ticket,
        nearby_tickets,
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Input) -> usize {
    input
        .nearby_tickets
        .iter()
        .flat_map(|ticket| ticket.0.iter())
        .fold(0, |acc, value| {
            if input.rules.iter().any(|r| r.contains(*value)) {
                acc
            } else {
                acc + value
            }
        })
}

fn part2_details(input: &Input) -> HashMap<&str, usize> {
    // O(tickets * rules)
    // Find all tickets that are valid.
    let valid_tickets = input
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .0
                .iter()
                .all(|value| input.rules.iter().any(|r| r.contains(*value)))
        })
        .collect::<Vec<_>>();

    // O(rules * rules * tickets)
    // Create pairs of rows to vec of all possibly valid columns.
    let num_columns = input.rules.len();
    let mut valid_columns = input
        .rules
        .iter()
        .map(|rule| {
            (
                rule,
                (0..num_columns)
                    .filter(|i| {
                        valid_tickets
                            .iter()
                            .all(|ticket| rule.contains(ticket.0[*i]))
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    // Iterate to a fixed point... how to improve this? Could this ever inf loop?
    while valid_columns.iter().any(|(_, columns)| columns.len() > 1) {
        // Find all columns that only contain one element
        let unique_columns = valid_columns
            .iter()
            .filter_map(|(_, columns)| {
                if columns.len() == 1 {
                    Some(columns[0])
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // O(rules * rules)
        for column in unique_columns {
            for (_, columns) in valid_columns
                .iter_mut()
                .filter(|(_, columns)| columns.len() > 1)
            {
                if let Some(index) = columns
                    .iter()
                    .enumerate()
                    .find(|(_, &c)| c == column)
                    .map(|(i, _)| i)
                {
                    columns.swap_remove(index);
                }
            }
        }
    }

    valid_columns
        .into_iter()
        .map(|(rule, columns)| (rule.name.as_str(), columns[0]))
        .collect()
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let columns = part2_details(input);
    columns
        .into_iter()
        .filter(|(key, _)| key.starts_with("departure"))
        .map(|(_, i)| input.your_ticket.0[i])
        .product()
}

#[test]
fn test_input1() {
    let content = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 71);
}

#[test]
fn test_input2() {
    let content = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
    let input = input_generator(content);
    let result = part2_details(&input);
    assert_eq!(input.your_ticket.0[result["class"]], 12);
    assert_eq!(input.your_ticket.0[result["row"]], 11);
    assert_eq!(input.your_ticket.0[result["seat"]], 13);
}
