use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Copy, Clone)]
pub struct Move {
    dir: Direction,
    amount: i32,
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            Move {
                dir: match chars.next() {
                    Some('N') => Direction::North,
                    Some('E') => Direction::East,
                    Some('S') => Direction::South,
                    Some('W') => Direction::West,
                    Some('L') => Direction::Left,
                    Some('R') => Direction::Right,
                    Some('F') => Direction::Forward,
                    i => panic!("Unexpected input: {:?}", i),
                },
                amount: chars
                    .collect::<String>()
                    .parse()
                    .expect("Expected string to end in a number"),
            }
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Move]) -> usize {
    let forward_options = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut forward_index = 0;
    let mut location = (0, 0);
    for r#move in input {
        match r#move.dir {
            Direction::North => location.1 += r#move.amount,
            Direction::East => location.0 += r#move.amount,
            Direction::South => location.1 -= r#move.amount,
            Direction::West => location.0 -= r#move.amount,
            Direction::Left => {
                forward_index = ((forward_index as isize - r#move.amount as isize / 90)
                    + forward_options.len() as isize) as usize
                    % forward_options.len()
            }
            Direction::Right => {
                forward_index =
                    (forward_index + r#move.amount as usize / 90) % forward_options.len()
            }
            Direction::Forward => {
                let movement = forward_options[forward_index];
                location.0 += movement.0 * r#move.amount;
                location.1 += movement.1 * r#move.amount;
            }
        }
    }

    (location.0.abs() + location.1.abs()) as usize
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Move]) -> usize {
    let mut waypoint = (10, 1);
    let mut location = (0, 0);
    for r#move in input {
        match r#move.dir {
            Direction::North => waypoint.1 += r#move.amount,
            Direction::East => waypoint.0 += r#move.amount,
            Direction::South => waypoint.1 -= r#move.amount,
            Direction::West => waypoint.0 -= r#move.amount,
            // Fun fact: Bastardized 2D rotation matrix
            Direction::Left => {
                for _ in 0..r#move.amount / 90 {
                    waypoint = (-1 * waypoint.1, waypoint.0);
                }
            }
            Direction::Right => {
                for _ in 0..r#move.amount / 90 {
                    waypoint = (waypoint.1, -1 * waypoint.0);
                }
            }
            Direction::Forward => {
                location.0 += waypoint.0 * r#move.amount;
                location.1 += waypoint.1 * r#move.amount;
            }
        }
    }

    (location.0.abs() + location.1.abs()) as usize
}

#[test]
fn test_input1() {
    let content = "F10
N3
F7
R90
F11";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 25);
}

#[test]
fn test_input2() {
    let content = "F10
N3
F7
R90
F11";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 286);
}
