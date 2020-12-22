use std::collections::*;

fn parse_group(s: &str) -> i32 {
    s.split_ascii_whitespace()
        .flat_map(|s| s.chars())
        .fold(HashSet::new(), |mut acc, c| {
            acc.insert(c);
            acc
        })
        .len() as i32
}

#[test]
fn parse_group_test() {
    assert_eq!(3, parse_group("abc"));
    assert_eq!(3, parse_group("a\nb\nc"));
    assert_eq!(3, parse_group("ab\nac"));
    assert_eq!(1, parse_group("a\na\na\na"));
    assert_eq!(1, parse_group("b"));
    assert_eq!(6, parse_group("abcx\nabcy\nabcz"));
}

fn part1(input: &str) -> i32 {
    let groups = input.split("\n\n");
    groups.map(|s| parse_group(s) as i32).sum()
}

#[test]
fn day06_input_with_groups() {
    assert_eq!(2, part1("a\n\na"));
    assert_eq!(4, part1("ab\n\nac"));
}

fn parse_unique_answers(s: &str) -> i32 {
    let line_count = s.lines().count();

    let m =
        s.split_ascii_whitespace()
            .flat_map(|s| s.chars())
            .fold(HashMap::new(), |mut acc, c| {
                let entry = acc.entry(c).or_insert(0);
                *entry += 1;
                acc
            });
    // m: [(answer, count)]
    m.into_iter()
        .filter(|(k, v)| *v == line_count)
        .count() as i32
}

#[test]
fn test_part2() {
    assert_eq!(3, parse_unique_answers("abc"));
    assert_eq!(0, parse_unique_answers("a\nb\nc"));
    assert_eq!(1, parse_unique_answers("ab\nac"));

    let input = "abc

    a
    b
    c

    ab
    ac

    a
    a
    a
    a

    b";
    assert_eq!(6, part2(input));
}

fn part2(s: &str) -> i32 {
    s.split("\n\n").map(|s| parse_unique_answers(s)).sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part1: {}", part1(input));
    println!("Partd: {}", part2(input));
}
