use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

// Runs O(n + n*log n)
fn is_valid_range(input: &[usize], value: usize) -> bool {
    // Sort in O(n * log n).
    let input = {
        let mut input = input.iter().copied().collect::<Vec<_>>();
        input.sort();
        input
    };

    // Sliding window calculations is O(n).
    let mut left = 0;
    let mut right = 1;
    while right < input.len() {
        let sum = input[left] + input[right];
        if sum == value {
            return true;
        }

        if sum < value {
            right += 1;
        } else {
            left += 1;
        }

        if left == right {
            right += 1;
        }
    }

    false
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

// Since preamble_size is always constant, Runtime is O(n) :)
fn actually_solve_part1(input: &[usize], preamble_size: usize) -> usize {
    // Loop is O(n)
    for i in preamble_size..input.len() {
        // is_valid_range is O(preamble_size + preamble_size*log preamble_size) but we always call
        // with constant size of preamble, so it's also constant.
        if !is_valid_range(&input[i - preamble_size..i], input[i]) {
            return input[i];
        }
    }

    panic!("Value not found");
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    const PREAMBLE_SIZE: usize = 25;
    actually_solve_part1(input, PREAMBLE_SIZE)
}

// Runs in O(n)
fn actually_solve_part2(input: &[usize], preamble_size: usize) -> usize {
    // Runs in O(n). See comment in `actually_solve_part1`.
    let mut expected = 0;
    for i in preamble_size..input.len() {
        if !is_valid_range(&input[i - preamble_size..i], input[i]) {
            expected = input[i];
            break;
        }
    }

    // Sliding window runs in O(n).
    let mut left = 0;
    let mut right = 1;
    let mut sum: usize = input[left..=right].iter().sum();
    while right < input.len() {
        if sum == expected {
            let (min, max) = input[left..=right].iter().fold((std::usize::MAX, 0), |(min, max), &val| {
                if val < min {
                    (val, max)
                } else if val > max {
                    (min, val)
                } else {
                    (min, max)
                }
            });
            return min + max;
        }

        if sum > expected {
            sum -= input[left];
            left += 1;
        } else {
            right += 1;
            sum += input[right];
        }

        if right - left < 2 {
            right += 1;
            if right < input.len() {
                sum += input[right];
            }
        }
    }

    return expected;
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    const PREAMBLE_SIZE: usize = 25;
    actually_solve_part2(input, PREAMBLE_SIZE)
}

#[test]
fn test_input1() {
    let content = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    let input = input_generator(content);
    let result = actually_solve_part1(&input, 5);
    assert_eq!(result, 127);
}

#[test]
fn test_input2() {
    let content = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    let input = input_generator(content);
    let result = actually_solve_part2(&input, 5);
    assert_eq!(result, 62);
}
