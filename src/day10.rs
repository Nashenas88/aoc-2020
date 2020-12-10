use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashSet;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let mut input: Vec<_> = input.iter().copied().chain(Some(0).into_iter()).collect();
    input.sort();
    input.push(input[input.len() - 1] + 3);

    let (ones, threes) =
        input
            .windows(2)
            .map(|arr| arr[1] - arr[0])
            .fold((0, 0), |(ones, threes), diff| {
                if diff == 1 {
                    (ones + 1, threes)
                } else if diff == 3 {
                    (ones, threes + 1)
                } else {
                    (ones, threes)
                }
            });

    ones * threes
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[usize]) -> u128 {
    let mut input: Vec<_> = input.iter().copied().chain(Some(0).into_iter()).collect();
    input.sort();
    input.push(input[input.len() - 1] + 3);

    let mut res: Vec<u128> = vec![0; input.len()];
    res[0] = 1;
    res[1] = 1;
    res[2] = if input[2] - input[0] < 4 { 2 } else { 1 };

    for (i, arr) in input.windows(4).enumerate() {
        let last = arr[3];
        for (j, v) in arr[..3].into_iter().enumerate() {
            if last - *v < 4 {
                res[i + 3] += res[i + j];
            }
        }
    }

    res[input.len() - 1]
}

#[test]
fn test_input1() {
    let content = "16
10
15
5
1
11
7
19
6
12
4";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 35);
}

#[test]
fn test_input1_2() {
    let content = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 220);
}

#[test]
fn test_input2() {
    // 4 5 6 7 is the tricky one here...
    // 5 -> 7 => 2 chances, but NOT when
    // 4 -> 6 or 4 -> 7, since NO 5 ...
    let content = "16
10
15
5
1
11
7
19
6
12
4";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 8);
}

#[test]
fn test_input2_2() {
    let content = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 19208);
}
