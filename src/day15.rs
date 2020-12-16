use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;

struct Generator {
    map: HashMap<usize, usize>,
    idx: usize,
    last: usize,
}

impl Generator {
    pub fn new(init: &[usize]) -> Self {
        Self {
            map: init
                .iter()
                .copied()
                .take(init.len() - 1)
                .enumerate()
                .map(|(i, v)| (v, i))
                .collect(),
            idx: init.len() - 1,
            last: init[init.len() - 1],
        }
    }

    pub fn next(&mut self) -> usize {
        let res = match self.map.get(&self.last) {
            Some(idx) => self.idx - idx,
            None => 0,
        };
        self.map.insert(self.last, self.idx);
        self.idx += 1;
        self.last = res;
        res
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    const GENERATIONS: usize = 2020;
    let mut generator = Generator::new(input);
    let mut res = 0;
    for _ in 0..(GENERATIONS - input.len()) {
        res = generator.next();
    }

    res
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    const GENERATIONS: usize = 30000000;
    let mut generator = Generator::new(input);
    let mut res = 0;
    for _ in 0..(GENERATIONS - input.len()) {
        res = generator.next();
    }

    res
}

#[test]
fn test_input1_1() {
    let content = "0,3,6";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 436);
}

#[test]
fn test_input1_2() {
    let content = "1,3,2";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 1);
}

#[test]
fn test_input1_3() {
    let content = "2,1,3";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 10);
}

#[test]
fn test_input1_4() {
    let content = "1,2,3";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 27);
}

#[test]
fn test_input1_5() {
    let content = "2,3,1";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 78);
}

#[test]
fn test_input1_6() {
    let content = "3,2,1";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 438);
}

#[test]
fn test_input1_7() {
    let content = "3,1,2";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 1836);
}

#[test]
fn test_input2_1() {
    let content = "0,3,6";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 175594);
}

#[test]
fn test_input2_2() {
    let content = "1,3,2";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 2578);
}

#[test]
fn test_input2_3() {
    let content = "2,1,3";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 3544142);
}

#[test]
fn test_input2_4() {
    let content = "1,2,3";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 261214);
}

#[test]
fn test_input2_5() {
    let content = "2,3,1";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 6895259);
}

#[test]
fn test_input2_6() {
    let content = "3,2,1";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 18);
}

#[test]
fn test_input2_7() {
    let content = "3,1,2";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 362);
}
