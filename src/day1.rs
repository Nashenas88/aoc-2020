use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    let mut input = input
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect::<Vec<_>>();
    input.sort();
    input
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let (mut left, mut right) = (0, 1);
    while left < input.len() - 1 && right < input.len() {
        let left_val = input[left];
        let right_val = input[right];
        let sum = left_val + right_val;
        if sum == 2020 {
            return left_val * right_val;
        }

        if sum > 2020 {
            left += 1;
            right = left + 1;
        } else {
            right += 1;
        }
    }

    0
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let (mut left, mut middle, mut right) = (0, 1, 2);
    while left < input.len() - 2 {
        let left_val = input[left];

        while middle < input.len() - 1 && right < input.len() {
            let middle_val = input[middle];
            let right_val = input[right];

            let sum = left_val + middle_val + right_val;
            if sum == 2020 {
                return left_val * middle_val * right_val;
            }

            if sum > 2020 {
                middle += 1;
                right = middle + 1;
            } else {
                right += 1;
            }
        }

        left += 1;
        middle = left + 1;
        right = middle + 1;
    }

    0
}
