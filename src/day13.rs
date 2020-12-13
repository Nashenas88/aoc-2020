use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::num::NonZeroU32;

pub struct Schedule {
    earliest: usize,
    bus_ids: Vec<Option<NonZeroU32>>,
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Schedule {
    let mut lines = input.lines();
    Schedule {
        earliest: lines.next().unwrap().parse().unwrap(),
        bus_ids: lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| {
                if s == "x" {
                    None
                } else {
                    s.parse::<NonZeroU32>().ok()
                }
            })
            .collect(),
    }
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Schedule) -> usize {
    let (id, wait_time) = input
        .bus_ids
        .iter()
        .filter_map(|id| id.as_ref().copied())
        .map(|id| {
            let id = id.get() as usize;
            (id, id - (input.earliest % id))
        })
        .min_by_key(|&(_, wait_time)| wait_time)
        .unwrap();

    id * wait_time
}

/// Find minimum x that satisfies x = p⁻¹ (mod q). This function does not
/// guarantee a unique result if `p` and `q` are not coprime.
/// ```rust,no_run
/// assert_eq!(5 * modulo_inverse(5, 7) % 7, 1);
/// ```
fn modulo_inverse(p: u128, q: u128) -> u128 {
    (1..q)
        .find(|i| (i * p) % q == 1)
        .expect("p and q must not be coprime")
}

#[test]
fn test_modulo_inverse() {
    let x = modulo_inverse(5, 7);
    assert_eq!(x, 3);
    assert_eq!(5 * x % 7, 1);
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Schedule) -> u128 {
    let pos_and_ids = input
        .bus_ids
        .iter()
        .enumerate()
        .filter_map(|(pos, id)| id.map(|id| (pos as u128, id.get() as u128)))
        .collect::<Vec<_>>();

    // Chinese remainder theorem https://crypto.stanford.edu/pbc/notes/numbertheory/crt.html
    // x = Σⁿᵢ₌₁(aᵢ * bᵢ * bᵢ') (mod M)
    // where M = Πⁿᵢ₌₁mᵢ
    //       bᵢ = M / mᵢ
    //       bᵢ' = bᵢ⁻¹ (mod mᵢ)
    let big_m: u128 = pos_and_ids.iter().map(|&(_, id)| id as u128).product();
    let sum: u128 = pos_and_ids
        .iter()
        .map(|&(pos, m)| {
            // Here we substract the `pos` from `m` to get `a` because if we just used
            // the original `pos`, we would get the inverse order for some time t.
            let a = m - pos;
            let b = big_m / m;
            let b_prime = modulo_inverse(b, m);
            assert_eq!(b * b_prime % m, 1);
            a * b * b_prime
        })
        .sum();
    sum % big_m
}

#[test]
fn test_input1() {
    let content = "939
7,13,x,x,59,x,31,19";
    let input = input_generator(content);
    let result = solve_part1(&input);
    assert_eq!(result, 295);
}

#[test]
fn test_input2_1() {
    let content = "1068781
7,13,x,x,59,x,31,19";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 1068781);
}

#[test]
fn test_input2_2() {
    let content = "3417
17,x,13,19";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 3417);
}

#[test]
fn test_input2_3() {
    let content = "0
67,7,59,61";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 754018);
}

#[test]
fn test_input2_4() {
    let content = "0
67,x,7,59,61";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 779210);
}

#[test]
fn test_input2_5() {
    let content = "0
67,7,x,59,61";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 1261476);
}

#[test]
fn test_input2_6() {
    let content = "0
1789,37,47,1889";
    let input = input_generator(content);
    let result = solve_part2(&input);
    assert_eq!(result, 1202161486);
}
