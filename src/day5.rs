use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(Copy, Clone)]
pub struct Seat {
    row: u8,
    column: u8,
}

impl Seat {
    fn id(&self) -> u32 {
        (self.row as u32) * 8 + (self.column as u32)
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Seat> {
    input
        .lines()
        .map(|line| {
            let mut left = 0;
            let mut right = 127;
            let mut char_iter = line.chars();
            for _ in 0..7 {
                let c = char_iter.next().unwrap();
                match c {
                    'F' => right = (right - left) / 2 + left,
                    'B' => left = (right - left) / 2 + left + 1,
                    _ => panic!("Unexpected char {}", c),
                }
            }
            assert_eq!(left, right);
            let row = left;

            left = 0;
            right = 7;
            for _ in 0..3 {
                let c = char_iter.next().unwrap();
                match c {
                    'R' => left = (right - left) / 2 + left + 1,
                    'L' => right = (right - left) / 2 + left,
                    _ => panic!("Unexpected char {}", c),
                }
            }
            assert_eq!(left, right);
            let column = left;

            Seat { row, column }
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Seat]) -> u32 {
    input.iter().map(|s| s.id()).max().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Seat]) -> u32 {
    let mut seats = input.iter().cloned().collect::<Vec<_>>();
    seats.sort_by_key(|s| s.id());
    for window in seats.windows(2) {
        if let [left, right] = window {
            if left.id() == right.id() - 2 {
                return left.id() + 1;
            }
        }
    }

    panic!("I can't find my seat ‧º·(˚ ˃̣̣̥⌓˂̣̣̥ )‧º·˚");
}

#[test]
fn seat_test_1() {
    let content = "FBFBBFFRLR";
    let input = input_generator(content);
    let seat = &input[0];
    assert_eq!(seat.row, 44);
    assert_eq!(seat.column, 5);
    assert_eq!(seat.id(), 357);
}

#[test]
fn seat_test_2() {
    let content = "BFFFBBFRRR";
    let input = input_generator(content);
    let seat = &input[0];
    assert_eq!(seat.row, 70);
    assert_eq!(seat.column, 7);
    assert_eq!(seat.id(), 567);
}

#[test]
fn seat_test_3() {
    let content = "FFFBBBFRRR";
    let input = input_generator(content);
    let seat = &input[0];
    assert_eq!(seat.row, 14);
    assert_eq!(seat.column, 7);
    assert_eq!(seat.id(), 119);
}

#[test]
fn seat_test_4() {
    let content = "BBFFBBFRLL";
    let input = input_generator(content);
    let seat = &input[0];
    assert_eq!(seat.row, 102);
    assert_eq!(seat.column, 4);
    assert_eq!(seat.id(), 820);
}
