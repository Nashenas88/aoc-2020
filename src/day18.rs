use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Token {
    Num(usize),
    Op(Op),
    Paren(Vec<Token>),
}

#[derive(Copy, Clone, Debug)]
pub enum Op {
    Add,
    Mul,
}

fn input_generator_helper(input: &str) -> (Vec<Token>, usize) {
    let mut idx = 0;
    let mut tokens = vec![];
    let map = input.char_indices().collect::<HashMap<_, _>>();

    while idx < input.bytes().len() {
        let c = map.get(&idx).unwrap();
        let mut new_idx = idx + c.len_utf8();
        let token = match c {
            '0'..='9' => Token::Num(c.to_digit(10).unwrap() as usize),
            '+' => Token::Op(Op::Add),
            '*' => Token::Op(Op::Mul),
            '(' => {
                let (tokens, tokens_read) = input_generator_helper(input.get(new_idx..).unwrap());
                new_idx += tokens_read;
                Token::Paren(tokens)
            }
            ')' => return (tokens, new_idx),
            _ => {
                idx = new_idx;
                continue;
            }
        };
        tokens.push(token);
        idx = new_idx;
    }

    (tokens, input.len())
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Vec<Token>> {
    input
        .lines()
        .map(|line| {
            let (result, remainder) = input_generator_helper(line);
            assert_eq!(remainder, line.len());
            result
        })
        .collect()
}

fn update_result(op_stack: &mut Vec<Op>, left: usize, right: usize) -> usize {
    let op = op_stack
        .pop()
        .expect("Operators must come between numbers and/or parens");
    match op {
        Op::Add => left + right,
        Op::Mul => left * right,
    }
}

fn solve_part1_line(input: &[Token]) -> usize {
    let mut result = match &input[0] {
        &Token::Num(num) => num,
        Token::Paren(tokens) => solve_part1_line(tokens),
        Token::Op(op) => panic!(
            "Operator `{}` cannot be the first token!",
            if let Op::Add = op { '+' } else { '*' }
        ),
    };
    let mut op_stack = vec![];
    for token in &input[1..] {
        match token {
            &Token::Num(num) => {
                result = update_result(&mut op_stack, result, num);
            }
            &Token::Op(op) => op_stack.push(op),
            Token::Paren(tokens) => {
                result = update_result(&mut op_stack, result, solve_part1_line(tokens));
            }
        }
    }

    result
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[Vec<Token>]) -> usize {
    input.iter().map(|line| solve_part1_line(line)).sum()
}

#[derive(Debug)]
enum Node {
    Num(usize),
    Op(Box<OpNode>),
}

impl Node {
    fn eval(&self) -> usize {
        match self {
            Node::Num(num) => *num,
            Node::Op(op_node) => {
                let left = op_node.left.as_ref().expect("Bad tree").eval();
                let right = op_node.right.as_ref().expect("Bad tree").eval();
                match op_node.op {
                    Op::Add => left + right,
                    Op::Mul => left * right,
                }
            }
        }
    }
}

#[derive(Debug)]
struct OpNode {
    op: Op,
    left: Option<Node>,
    right: Option<Node>,
}

fn get_stack(input: &[Token]) -> Node {
    let mut stack: Vec<Node> = vec![];
    let mut pop_next: Option<OpNode> = None;
    for token in input {
        match token {
            &Token::Num(num) => {
                if let Some(mut op_node) = pop_next.take() {
                    op_node.left = stack.pop();
                    op_node.right = Some(Node::Num(num));
                    stack.push(Node::Op(Box::new(op_node)));
                } else {
                    stack.push(Node::Num(num));
                }
            }
            &Token::Op(op @ Op::Add) => {
                pop_next = Some(OpNode {
                    op,
                    left: None,
                    right: None,
                });
            }
            &Token::Op(op @ Op::Mul) => stack.push(Node::Op(Box::new(OpNode {
                op,
                left: None,
                right: None,
            }))),
            Token::Paren(tokens) => {
                let node = get_stack(tokens);
                if let Some(mut op_node) = pop_next.take() {
                    op_node.left = stack.pop();
                    op_node.right = Some(node);
                    stack.push(Node::Op(Box::new(op_node)));
                } else {
                    stack.push(node);
                }
            }
        }
    }

    while stack.len() > 1 {
        let mut new_stack = vec![];
        let mut pop_next: Option<OpNode> = None;
        for mut node in stack {
            if let Some(mut op_node) = pop_next.take() {
                op_node.right = Some(node);
                new_stack.push(Node::Op(Box::new(op_node)));
                continue;
            }

            let mut should_move = false;
            if let Node::Op(ref mut op) = node {
                if op.left.is_none() {
                    op.left = new_stack.pop();
                }

                should_move = op.right.is_none();
            }

            if should_move {
                if let Node::Op(op) = node {
                    pop_next = Some(*op);
                }

                continue;
            }

            new_stack.push(node)
        }
        stack = new_stack;
    }

    stack.into_iter().next().unwrap()
}

fn solve_part2_line(input: &[Token]) -> usize {
    let node = get_stack(input);
    node.eval()
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &[Vec<Token>]) -> usize {
    input.iter().map(|line| solve_part2_line(line)).sum()
}

#[test]
fn test_input1_1() {
    let content = "1 + 2 * 3 + 4 * 5 + 6";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 71);
}

#[test]
fn test_input1_2() {
    let content = "1 + (2 * 3) + (4 * (5 + 6))";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 51);
}

#[test]
fn test_input1_3() {
    let content = "2 * 3 + (4 * 5)";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 26);
}

#[test]
fn test_input1_4() {
    let content = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 437);
}

#[test]
fn test_input1_5() {
    let content = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 12240);
}

#[test]
fn test_input1_6() {
    let content = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 13632);
}

#[test]
fn test_input2_1() {
    let content = "1 + 2 * 3 + 4 * 5 + 6";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 231);
}

#[test]
fn test_input2_2() {
    let content = "1 + (2 * 3) + (4 * (5 + 6))";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 51);
}

#[test]
fn test_input2_3() {
    let content = "2 * 3 + (4 * 5)";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 46);
}

#[test]
fn test_input2_4() {
    let content = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 1445);
}

#[test]
fn test_input2_5() {
    let content = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 669060);
}

#[test]
fn test_input2_6() {
    let content = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 23340);
}

#[test]
fn test_input2_7() {
    let content = "2 + 3 * 4 * 5 * 2 + 2";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 400);
}
