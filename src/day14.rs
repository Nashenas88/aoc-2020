use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use regex::Regex;
use std::collections::HashMap;

pub struct Mask {
    ones_mask: u64,
    zeros_mask: u64,
    floating_ones: Vec<u64>,
    floating_zeros: Vec<u64>,
}

pub struct Set {
    address: u64,
    value: u64,
}

pub enum Op {
    Mask(Mask),
    Set(Set),
}

struct Emulator<'a> {
    ones_mask: u64,
    zeros_mask: u64,
    floating_ones: &'a[u64],
    floating_zeros: &'a [u64],
    pc: usize,
    instructions: &'a [Op],
    memory: HashMap<u64, u64>,
}

enum OperatingMode {
    ValueMask,
    MemoryMask,
}

impl<'a> Emulator<'a> {
    pub fn new(instructions: &'a [Op]) -> Self {
        Self {
            ones_mask: 0,
            zeros_mask: !0 << SHIFT_CORRECTION >> SHIFT_CORRECTION,
            floating_ones: &[],
            floating_zeros: &[],
            pc: 0,
            instructions: &instructions,
            memory: HashMap::new(),
        }
    }

    pub fn step(&mut self, op_mode: OperatingMode) -> bool {
        if self.pc >= self.instructions.len() {
            return false;
        }

        match &self.instructions[self.pc] {
            Op::Mask(Mask {
                ones_mask,
                zeros_mask,
                floating_ones,
                floating_zeros,
            }) => {
                self.ones_mask = *ones_mask;
                self.zeros_mask = *zeros_mask;
                self.floating_ones = &floating_ones;
                self.floating_zeros = &floating_zeros;
            }
            Op::Set(Set { address, value }) => {
                match op_mode {
                    OperatingMode::ValueMask => {
                        self.memory
                            .insert(*address, value & self.zeros_mask | self.ones_mask);
                    }
                    OperatingMode::MemoryMask => {
                        for (&floating_one, &floating_zero) in
                            self.floating_ones.iter().zip(self.floating_zeros.iter())
                        {
                            self.memory
                                .insert(address & floating_zero | floating_one, *value);
                        }
                    }
                };
            }
        };
        self.pc += 1;
        true
    }

    pub fn memory_sum(&self) -> usize {
        self.memory.values().map(|&v| v as usize).sum()
    }
}

const HIGH_BIT: u64 = 35;
const SHIFT_CORRECTION: u64 = 64 - 36;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Op> {
    let re = Regex::new(r"^mem\[([0-9]+)\] = ([0-9]+)$").unwrap();
    input
        .lines()
        .map(|line| {
            if line.starts_with("mask") {
                let mask = line.split(' ').skip(2).next().unwrap();
                let (zeros_mask, ones_mask, floating_zeros, floating_ones) =
                    mask.chars().enumerate().fold(
                        (0, 0, vec![0], vec![0]),
                        |(zeros, ones, floating_zeros, floating_ones), (i, c)| {
                            let val = 1 << (HIGH_BIT - i as u64);
                            let neg_val = !val << SHIFT_CORRECTION >> SHIFT_CORRECTION;
                            match c {
                                '1' => (
                                    val | zeros,
                                    val | ones,
                                    floating_zeros.into_iter().map(|v| val | v).collect(),
                                    floating_ones.into_iter().map(|v| val | v).collect(),
                                ),
                                '0' => (
                                    neg_val & zeros,
                                    neg_val & ones,
                                    floating_zeros.into_iter().map(|v| val | v).collect(),
                                    floating_ones.into_iter().map(|v| neg_val & v).collect(),
                                ),
                                _ => (
                                    val | zeros,
                                    neg_val & ones,
                                    floating_zeros
                                        .into_iter()
                                        .flat_map(|v| vec![val | v, neg_val & v])
                                        .collect(),
                                    floating_ones
                                        .into_iter()
                                        .flat_map(|v| vec![val | v, neg_val & v])
                                        .collect(),
                                ),
                            }
                        },
                    );

                Op::Mask(Mask {
                    zeros_mask,
                    ones_mask,
                    floating_ones,
                    floating_zeros,
                })
            } else {
                let captures = re
                    .captures(line)
                    .expect(&format!("Could not process `{}`", line));
                Op::Set(Set {
                    address: captures[1].parse::<u64>().unwrap(),
                    value: captures[2].parse::<u64>().unwrap(),
                })
            }
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[Op]) -> usize {
    let mut emulator = Emulator::new(input);
    while emulator.step(OperatingMode::ValueMask) {}
    emulator.memory_sum()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &[Op]) -> usize {
    let mut emulator = Emulator::new(input);
    while emulator.step(OperatingMode::MemoryMask) {}
    emulator.memory_sum()
}

#[test]
fn test_input1() {
    let content = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 165);
}

#[test]
fn test_input2() {
    let content = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 208);
}
