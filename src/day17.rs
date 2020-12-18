use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::{HashMap, HashSet};

pub struct Input {
    live_locations: Vec<Point>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point(isize, isize, isize, isize);

impl Point {
    fn neighbors_3d(&self) -> Neighbors<'_> {
        Neighbors::new(self)
    }

    fn neighbors_4d(&self) -> Neighbors4D<'_> {
        Neighbors4D::new(self)
    }
}

struct Neighbors<'a> {
    point: &'a Point,
    i: isize,
    j: isize,
    k: isize,
}

impl<'a> Neighbors<'a> {
    fn new(point: &'a Point) -> Self {
        Self {
            point,
            i: point.0 - 1,
            j: point.1 - 1,
            k: point.2 - 1,
        }
    }
}

impl<'a> Iterator for Neighbors<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.k > self.point.2 + 1 {
            self.j += 1;
            self.k = self.point.2 - 1;
        }

        if self.j > self.point.1 + 1 {
            self.i += 1;
            self.j = self.point.1 - 1;
        }

        if self.i > self.point.0 + 1 {
            return None;
        }

        if self.i == self.point.0 && self.j == self.point.1 && self.k == self.point.2 {
            self.k += 1;
            return self.next();
        }

        let res = Point(self.i, self.j, self.k, 0);
        self.k += 1;
        Some(res)
    }
}

struct Neighbors4D<'a> {
    point: &'a Point,
    i: isize,
    j: isize,
    k: isize,
    l: isize,
}

impl<'a> Neighbors4D<'a> {
    fn new(point: &'a Point) -> Self {
        Self {
            point,
            i: point.0 - 1,
            j: point.1 - 1,
            k: point.2 - 1,
            l: point.3 - 1,
        }
    }
}

impl<'a> Iterator for Neighbors4D<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.l > self.point.3 + 1 {
            self.k += 1;
            self.l = self.point.3 - 1;
        }

        if self.k > self.point.2 + 1 {
            self.j += 1;
            self.k = self.point.2 - 1;
        }

        if self.j > self.point.1 + 1 {
            self.i += 1;
            self.j = self.point.1 - 1;
        }

        if self.i > self.point.0 + 1 {
            return None;
        }

        if self.i == self.point.0
            && self.j == self.point.1
            && self.k == self.point.2
            && self.l == self.point.3
        {
            self.l += 1;
            return self.next();
        }

        let res = Point(self.i, self.j, self.k, self.l);
        self.l += 1;
        Some(res)
    }
}

fn print_set(set: &HashSet<Point>) {
    let (min_x, min_y, min_z, min_w, max_x, max_y, max_z, max_w) = set.iter().fold(
        (
            std::isize::MAX,
            std::isize::MAX,
            std::isize::MAX,
            std::isize::MAX,
            std::isize::MIN,
            std::isize::MIN,
            std::isize::MIN,
            std::isize::MIN,
        ),
        |acc, p| {
            (
                if p.0 < acc.0 { p.0 } else { acc.0 },
                if p.1 < acc.1 { p.1 } else { acc.1 },
                if p.2 < acc.2 { p.2 } else { acc.2 },
                if p.3 < acc.3 { p.3 } else { acc.3 },
                if p.0 > acc.4 { p.0 } else { acc.4 },
                if p.1 > acc.5 { p.1 } else { acc.5 },
                if p.2 > acc.6 { p.2 } else { acc.6 },
                if p.3 > acc.7 { p.3 } else { acc.7 },
            )
        },
    );

    for w in min_w..=max_w {
        for z in min_z..=max_z {
            println!("z={}", z);
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    print!(
                        "{}",
                        if set.contains(&Point(x, y, z, w)) {
                            '#'
                        } else {
                            '.'
                        }
                    );
                }
                println!("");
            }
            println!("");
        }
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Input {
    Input {
        live_locations: input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    if c == '#' {
                        Some(Point(x as isize, y as isize, 0, 0))
                    } else {
                        None
                    }
                })
            })
            .filter_map(|c| c)
            .collect(),
    }
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let mut set = input.live_locations.iter().cloned().collect::<HashSet<_>>();

    for _ in 0..6 {
        let mut map = HashMap::new();
        for point in set.iter() {
            for neighbor in point.neighbors_3d() {
                *map.entry(neighbor).or_insert(0) += 1;
            }
        }

        set = map
            .into_iter()
            .filter_map(|(point, count)| {
                if count == 3 || (count == 2 && set.contains(&point)) {
                    Some(point)
                } else {
                    None
                }
            })
            .collect();
    }

    set.len()
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut set = input.live_locations.iter().cloned().collect::<HashSet<_>>();

    for _ in 0..6 {
        let mut map = HashMap::new();
        for point in set.iter() {
            for neighbor in point.neighbors_4d() {
                *map.entry(neighbor).or_insert(0) += 1;
            }
        }

        set = map
            .into_iter()
            .filter_map(|(point, count)| {
                if count == 3 || (count == 2 && set.contains(&point)) {
                    Some(point)
                } else {
                    None
                }
            })
            .collect();
    }

    set.len()
}

#[test]
fn test_input1() {
    let content = ".#.
..#
###";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 112);
}

#[test]
fn test_input2() {
    let content = ".#.
..#
###";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 848);
}
