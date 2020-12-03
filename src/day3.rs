use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

const fn next_right(width: usize, x: usize, right: usize) -> usize {
    (x + right) % width
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<bool>]) -> u32 {
    const START: usize = 0;
    const RIGHT: usize = 3;
    let mut pos = START;
    input
        .iter()
        .skip(1)
        .map(|line| {
            pos = next_right(line.len(), pos, RIGHT);
            line[pos] as u32
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<bool>]) -> u32 {
    const START: usize = 0;
    const TRAJECTORIES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    TRAJECTORIES
        .iter()
        .map(|&(right, down)| {
            let mut pos = START;
            dbg!(input
                .iter()
                .step_by(down)
                .map(|line| {
                    let res = line[pos] as u32;
                    pos = next_right(line.len(), pos, right);
                    res
                })
                .sum::<u32>())
        })
        .product()
}
