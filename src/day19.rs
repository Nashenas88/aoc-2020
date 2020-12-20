use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;

pub struct Input {
    rules_graph: HashMap<usize, CfgNode>,
    input_list: Vec<String>,
}

pub struct Graph {
    nodes: HashMap<usize, Node>,
}

impl Graph {
    fn matches_root(&self, input: &str) -> bool {
        #[allow(dead_code)]
        fn print_bools(bools: &Vec<Vec<HashMap<usize, bool>>>) {
            for v in bools.iter().rev() {
                for v in v.iter() {
                    if let Some(i) = v.iter().find_map(|(&i, &b)| if b { Some(i) } else { None }) {
                        print!("{:2}|", i);
                    } else {
                        print!(" .|");
                    }
                }
                println!("");
            }
        }

        let n = input.chars().count();
        let mut bools = vec![vec![HashMap::<usize, bool>::new(); n]; n];

        for (s, c) in input.chars().enumerate() {
            for (v, node_c) in self.nodes.iter().flat_map(|(i, node)| {
                node.choices.iter().filter_map(move |choice| {
                    if let &InnerNode::Char(c) = choice {
                        Some((i, c))
                    } else {
                        None
                    }
                })
            }) {
                if c == node_c {
                    bools[s][s].insert(*v, true);
                }
            }
        }

        // length of span
        for i in 2..=n {
            // Start of span
            for l in 1..=(n - i + 1) {
                let r = l + i - 1;
                for m in (l + 1)..=r {
                    for (a, choice) in self.nodes.iter().filter_map(|(i, node)| {
                        let sequence = node
                            .choices
                            .iter()
                            .filter_map(|choice| {
                                if let &InnerNode::Sequence(seq) = choice {
                                    Some(seq)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>();
                        if sequence.len() > 0 {
                            Some((i, sequence))
                        } else {
                            None
                        }
                    }) {
                        if choice.iter().any(|&[b, c]| {
                            let res = bools[l - 1][m - 2].get(&b).map(|b| *b).unwrap_or(false)
                                && bools[m - 1][r - 1].get(&c).map(|b| *b).unwrap_or(false);
                            // println!(
                            //     "[{}][{}][{}] && [{}][{}][{}] = {}",
                            //     (l - 1),
                            //     (m - 2),
                            //     b,
                            //     (m - 1),
                            //     (r - 1),
                            //     c,
                            //     res
                            // );
                            res
                        }) {
                            bools[l - 1][r - 1].insert(*a, true);
                        }
                    }
                }
            }
        }

        bools[0][n - 1].get(&0).map(|b| *b).unwrap_or(false)
    }
}

pub enum InnerNode {
    Char(char),
    Sequence([usize; 2]),
}

pub struct Node {
    choices: Vec<InnerNode>,
}

#[derive(Clone)]
enum CfgInnerNode {
    Char(char),
    Sequence(Vec<usize>),
}

#[derive(Clone)]
struct CfgNode {
    choices: Vec<CfgInnerNode>,
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let mut next_line = lines.next();
    let mut nodes = HashMap::new();

    while next_line.map(|l| l.len() != 0).unwrap_or(false) {
        let line = next_line.unwrap();
        let mut parts = line.split(": ");
        let index = parts.next().unwrap().parse::<usize>().unwrap();
        let rule = parts.next().unwrap();
        let node = if rule.starts_with('"') {
            CfgNode {
                choices: vec![CfgInnerNode::Char(rule.chars().skip(1).next().unwrap())],
            }
        } else {
            CfgNode {
                choices: rule
                    .split(" | ")
                    .into_iter()
                    .map(|rule| {
                        CfgInnerNode::Sequence(
                            rule.split(' ')
                                .into_iter()
                                .map(|idx| idx.parse::<usize>().unwrap())
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect(),
            }
        };
        nodes.insert(index, node);

        next_line = lines.next();
    }

    let input_list = lines.into_iter().map(String::from).collect();

    Input {
        rules_graph: nodes,
        input_list,
    }
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let mut nodes = input.rules_graph.clone();
    let mut next_index = *nodes.keys().max().unwrap() + 1;

    for i in nodes.keys().cloned().collect::<Vec<_>>() {
        let mut to_push = vec![];
        for choice in &mut nodes.get_mut(&i).unwrap().choices {
            if let CfgInnerNode::Sequence(choice) = choice {
                if choice.len() > 3 {
                    panic!("Need to add logic to handle more than 3... ugh");
                }

                if choice.len() == 3 {
                    let last = choice.pop().unwrap();
                    let middle = choice.pop().unwrap();
                    choice.push(next_index);
                    to_push.push((
                        next_index,
                        CfgNode {
                            choices: vec![CfgInnerNode::Sequence(vec![middle, last])],
                        },
                    ));
                    next_index += 1;
                }
            }
        }

        nodes.extend(to_push.into_iter())
    }

    for i in nodes.keys().cloned().collect::<Vec<_>>() {
        let mut to_lookup = vec![];
        for choice in &nodes[&i].choices {
            if let CfgInnerNode::Sequence(choice) = choice {
                if choice.len() == 1 {
                    to_lookup.push(choice[0]);
                }
            }
        }

        let map = to_lookup
            .into_iter()
            .flat_map(|i| nodes[&i].choices.clone())
            .collect::<Vec<_>>();

        nodes.get_mut(&i).unwrap().choices.retain(|node| {
            if let CfgInnerNode::Sequence(choice) = node {
                choice.len() > 1
            } else {
                true
            }
        });
        nodes.get_mut(&i).unwrap().choices.extend(map.into_iter());
    }

    let graph = Graph {
        nodes: nodes
            .iter()
            .map(|(&id, node)| (id, node.clone()))
            .map(|(i, node)| {
                (
                    i,
                    Node {
                        choices: node
                            .choices
                            .into_iter()
                            .map(|inner_node| match inner_node {
                                CfgInnerNode::Char(c) => InnerNode::Char(c),
                                CfgInnerNode::Sequence(v) => InnerNode::Sequence([v[0], v[1]]),
                            })
                            .collect(),
                    },
                )
            })
            .collect(),
    };

    input
        .input_list
        .iter()
        .filter(|line| graph.matches_root(line))
        .count()
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut nodes = input.rules_graph.clone();
    // Rules 8 and 11 are now recursive
    nodes.insert(
        8,
        CfgNode {
            choices: vec![
                CfgInnerNode::Sequence(vec![42]),
                CfgInnerNode::Sequence(vec![42, 8]),
            ],
        },
    );
    nodes.insert(
        11,
        CfgNode {
            choices: vec![
                CfgInnerNode::Sequence(vec![42, 31]),
                CfgInnerNode::Sequence(vec![42, 11, 31]),
            ],
        },
    );
    let mut next_index = *nodes.keys().max().unwrap() + 1;

    for i in nodes.keys().cloned().collect::<Vec<_>>() {
        let mut to_push = vec![];
        for choice in &mut nodes.get_mut(&i).unwrap().choices {
            if let CfgInnerNode::Sequence(choice) = choice {
                if choice.len() > 3 {
                    panic!("Need to add logic to handle more than 3... ugh");
                }

                if choice.len() == 3 {
                    let last = choice.pop().unwrap();
                    let middle = choice.pop().unwrap();
                    choice.push(next_index);
                    to_push.push((
                        next_index,
                        CfgNode {
                            choices: vec![CfgInnerNode::Sequence(vec![middle, last])],
                        },
                    ));
                    next_index += 1;
                }
            }
        }

        nodes.extend(to_push.into_iter())
    }

    for i in nodes.keys().cloned().collect::<Vec<_>>() {
        let mut to_lookup = vec![];
        for choice in &nodes[&i].choices {
            if let CfgInnerNode::Sequence(choice) = choice {
                if choice.len() == 1 {
                    to_lookup.push(choice[0]);
                }
            }
        }

        let map = to_lookup
            .into_iter()
            .flat_map(|i| nodes[&i].choices.clone())
            .collect::<Vec<_>>();

        nodes.get_mut(&i).unwrap().choices.retain(|node| {
            if let CfgInnerNode::Sequence(choice) = node {
                choice.len() > 1
            } else {
                true
            }
        });
        nodes.get_mut(&i).unwrap().choices.extend(map.into_iter());
    }

    let graph = Graph {
        nodes: nodes
            .iter()
            .map(|(&id, node)| (id, node.clone()))
            .map(|(i, node)| {
                (
                    i,
                    Node {
                        choices: node
                            .choices
                            .into_iter()
                            .map(|inner_node| match inner_node {
                                CfgInnerNode::Char(c) => InnerNode::Char(c),
                                CfgInnerNode::Sequence(v) => InnerNode::Sequence([v[0], v[1]]),
                            })
                            .collect(),
                    },
                )
            })
            .collect(),
    };

    input
        .input_list
        .iter()
        .filter(|line| graph.matches_root(line))
        .count()
}

#[test]
fn test_input1() {
    let content = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 2);
}

#[test]
fn test_input2() {
    let content = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 8);
}
