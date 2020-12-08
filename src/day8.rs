use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

pub enum Res {
    Loop(isize),
    Complete(isize),
    Pending,
}

#[derive(Copy, Clone)]
pub enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

pub struct Processor<'a> {
    pc: usize,
    acc: isize,
    instruction_counts: Vec<bool>,
    instructions: &'a [Instruction],
    swap: Option<usize>,
}

impl<'a> Processor<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            pc: 0,
            acc: 0,
            instruction_counts: vec![false; instructions.len()],
            instructions,
            swap: None,
        }
    }

    fn new_with_swap(instructions: &'a [Instruction], swap: usize) -> Self {
        Self {
            pc: 0,
            acc: 0,
            instruction_counts: vec![false; instructions.len()],
            instructions,
            swap: Some(swap),
        }
    }

    fn next(&mut self) -> Res {
        {
            let ref mut accessed = self.instruction_counts[self.pc];
            if *accessed {
                return Res::Loop(self.acc);
            }

            *accessed = true;
        }

        let op = match (self.instructions[self.pc], self.swap) {
            (Instruction::Nop(diff), Some(swap)) if swap == self.pc => {
                Instruction::Jmp(diff)
            }
            (Instruction::Jmp(diff), Some(swap)) if swap == self.pc => {
                Instruction::Nop(diff)
            }
            (op, _) => op,
        };

        match op {
            Instruction::Nop(_) => {
                self.pc += 1;
            },
            Instruction::Acc(diff) => {
                self.acc += diff;
                self.pc += 1;
            },
            Instruction::Jmp(diff) => {
                let pc = (self.pc as isize) + diff;
                assert!(pc >= 0, "program counter became negative");
                self.pc = pc as usize;
            },
        }

        if self.pc >= self.instructions.len() {
            Res::Complete(self.acc)
        } else {
            Res::Pending
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {

    input.lines().map(|line| {
        let mut split = line.split(' ');
        let op = split.next().expect("all lines should have one part");
        match op {
            "nop" => {
                let diff = split.next().expect("nop takes arg").parse().unwrap();
                Instruction::Nop(diff)
            }
            "acc" => {
                let diff = split.next().expect("acc takes arg").parse().unwrap();
                Instruction::Acc(diff)
            },
            "jmp" => {
                let diff = split.next().expect("jmp takes arg").parse().unwrap();
                Instruction::Jmp(diff)
            },
            _ => panic!("Unexpected variant {}", op),
        }
    }).collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Instruction]) -> isize {
    let mut processor = Processor::new(input);
    loop {
        if let Res::Loop(value) = processor.next() {
            return value;
        }
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Instruction]) -> isize {
    for i in 0..input.len() {
        if let Instruction::Acc(_) = input[i] {
            continue;
        }

        let mut processor = Processor::new_with_swap(input, i);
        loop {
            let result = processor.next();
            if let Res::Loop(_) = result {
                break;
            }

            if let Res::Complete(result) = result {
                println!("{}", i);
                return result;
            }
        }
    }

    -999
}

#[test]
fn test_input1() {
    let content ="nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 5);
}

#[test]
fn test_input2() {
    let content ="nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 8);
}
