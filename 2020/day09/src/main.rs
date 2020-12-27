use std::collections::HashSet;

fn parse(s: &str) -> Vec<i32> {
    s.lines().flat_map(|s| s.trim().parse::<i32>()).collect()
}

fn multiplications(n: &Vec<i32>) -> HashSet<i32> {
    n.iter()
        .flat_map(|&a| {
            let v: Vec<i32> = n
                .iter()
                .filter_map(|&b| if a == b { None } else { Some(a + b) })
                .collect();
            v.into_iter()
        })
        .collect()
}

#[test]
fn test_multiplications() {
    let seed = vec![1, 2, 3, 4];
    let expected: HashSet<_> = vec![3, 4, 5, 6, 7].iter().cloned().collect();
    let result = multiplications(&seed);
    assert_eq!(expected, result);
}

fn part1(s: &str, preamble: usize) -> i32 {
    let numbers = parse(s);
    let seed: Vec<i32> = numbers.iter().take(preamble).copied().collect();

    let invalid = numbers
        .iter()
        .skip(preamble)
        .scan(seed, |mut state, i| {
            println!("Visiting {} with state={:?}", i, state);
            let m = multiplications(state);
            let valid = m.contains(i);
            state.push(*i);
            state.remove(0);
            Some((i, valid))
        })
        .find(|(i, valid)| !valid);

    *invalid.unwrap().0
}
fn part2(s: &str) -> i32 {
    0
}

#[test]
fn test_part1_large() {
    let large = "35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576";

    let r = part1(large, 5);
    assert_eq!(127, r);
}
fn main() {
    let input = include_str!("input.txt");
    println!("Day 09");
    println!("Part 1: {:?}", part1(input, 25));
    println!("Part 2: {:?}", part2(input));
}
