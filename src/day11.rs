use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;

#[derive(Copy, Clone)]
pub enum Space {
    Floor,
    Empty,
    Occupied,
}

impl TryFrom<char> for Space {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Space::Floor,
            'L' => Space::Empty,
            '#' => Space::Occupied,
            _ => return Err(()),
        })
    }
}

struct Life {
    universe: Vec<Vec<Space>>,
}

impl Life {
    fn next_state(&mut self) -> bool {
        let mut board = self.universe.clone();
        let mut changed = false;
        let board_len = board.len();
        for (y, row) in board.iter_mut().enumerate() {
            let row_len = row.len();
            for (x, value) in row.iter_mut().enumerate() {
                let mut occupied_count = 0;

                for kernel_x in x.saturating_sub(1)..=(x + 1).min(row_len - 1) {
                    for kernel_y in y.saturating_sub(1)..=(y + 1).min(board_len - 1) {
                        if kernel_x == x && kernel_y == y {
                            continue;
                        }
                        if matches!(self.universe[kernel_y][kernel_x], Space::Occupied) {
                            occupied_count += 1;
                        }
                    }
                }

                match (value, occupied_count) {
                    (v @ Space::Empty, 0) => {
                        *v = Space::Occupied;
                        changed = true;
                    }
                    (v @ Space::Occupied, o) if o > 3 => {
                        *v = Space::Empty;
                        changed = true;
                    }
                    _ => {} // no-op
                }
            }
        }

        self.universe = board;
        changed
    }

    fn next_state2(&mut self) -> bool {
        let mut board = self.universe.clone();
        let mut changed = false;
        let board_len = board.len();
        for (y, row) in board.iter_mut().enumerate() {
            let row_len = row.len();
            for (x, value) in row.iter_mut().enumerate() {

                if matches!(value, Space::Floor) {
                    continue;
                }

                let mut occupied_count = 0;
                // mutable version for slopes in this one... should be improved

                for (&search_x, &search_y) in [-1, -1, 0, 1, 1, 1, 0, -1]
                    .iter()
                    .zip([0, 1, 1, 1, 0, -1, -1, -1].iter())
                {
                    let mut kernel_x = search_x + x as isize;
                    let mut kernel_y = search_y + y as isize;
                    while kernel_x >= 0
                        && kernel_x < row_len as isize
                        && kernel_y >= 0
                        && kernel_y < board_len as isize
                    {
                        match self.universe[kernel_y as usize][kernel_x as usize] {
                            Space::Floor => {
                                kernel_x += search_x;
                                kernel_y += search_y;
                            }
                            Space::Empty => {
                                break;
                            }
                            Space::Occupied => {
                                occupied_count += 1;
                                break;
                            }
                        }
                    }
                }

                match (value, occupied_count) {
                    (v @ Space::Empty, 0) => {
                        *v = Space::Occupied;
                        changed = true;
                    }
                    (v @ Space::Occupied, o) if o > 4 => {
                        *v = Space::Empty;
                        changed = true;
                    }
                    _ => {} // no-op
                }
            }
        }

        self.universe = board;
        changed
    }

    fn occupied(&self) -> usize {
        self.universe
            .iter()
            .map(|row| {
                row.iter()
                    .map(|value| {
                        if matches!(value, Space::Occupied) {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

impl Debug for Life {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = self
            .universe
            .iter()
            .map(|row| {
                row.iter()
                    .map(|space| match space {
                        Space::Floor => '.',
                        Space::Empty => 'L',
                        Space::Occupied => '#',
                    })
                    .collect::<String>()
                    + "\n"
            })
            .collect::<String>();
        write!(f, "{}", display)
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.try_into().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[Vec<Space>]) -> usize {
    let mut life = Life {
        universe: input.iter().cloned().collect(),
    };

    while life.next_state() {}
    life.occupied()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[Vec<Space>]) -> usize {
    let mut life = Life {
        universe: input.iter().cloned().collect(),
    };

    while life.next_state2() {}
    life.occupied()
}

#[test]
fn test_input1() {
    let content = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 37);
}

#[test]
fn test_input2() {
    let content = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 26);
}
