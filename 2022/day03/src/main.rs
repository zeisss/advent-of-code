#[derive(Debug, PartialEq, Clone)]
pub struct Rucksack {
    pub left: String,
    pub right: String,
}

mod parser {
    pub fn parse(input: &str) -> Vec<crate::Rucksack> {
        input
            .lines()
            .map(|line| {
                let len = line.len() / 2;
                crate::Rucksack {
                    left: line.chars().take(len).collect(),
                    right: line.chars().skip(len).collect(),
                }
            })
            .collect()
    }

    pub fn parse_priority(input: char) -> u32 {
        match input {
            'a'..='z' => input as u32 - 'a' as u32 + 1,
            'A'..='Z' => input as u32 - 'A' as u32 + 27,
            _ => panic!("unsupported input: {}", input),
        }
    }

    #[test]
    fn test_parse_priority() {
        assert_eq!(1, parse_priority('a'));
        assert_eq!(26, parse_priority('z'));
        assert_eq!(27, parse_priority('A'));
        assert_eq!(52, parse_priority('Z'));
    }

    #[test]
    fn test_parse() {
        let mut r = parse("abbc");
        assert_eq!(1, r.len());
        assert_eq!("ab", r[0].left);
        assert_eq!("bc", r[0].right);

        r = parse(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        );
        assert_eq!(2, r.len());
        assert_eq!("vJrwpWtwJgWr", r[0].left);
        assert_eq!("hcsFMMfFFhFp", r[0].right);
        assert_eq!("jqHRNqRjqzjGDLGL", r[1].left);
        assert_eq!("rsFMfFZSrLrFZsSL", r[1].right);
    }
}

mod logic {
    pub fn find_duplicates(left: &String, right: &String) -> Vec<char> {
        let mut v: Vec<char> = left
            .chars()
            .flat_map(|c| right.chars().filter(|c2| c == *c2).collect::<Vec<char>>())
            .collect();
        v.dedup();
        v
    }

    #[test]
    fn test_dups() {
        let input = "L".to_string();
        assert_eq!(vec!['L'], find_duplicates(&input, &input));

        assert_eq!(
            vec!['a'],
            find_duplicates(&"ab".to_string(), &"ca".to_string())
        );
        assert_eq!(
            vec!['a'],
            find_duplicates(&"aba".to_string(), &"ca".to_string())
        );
    }

    pub fn find_badge(input: &Vec<crate::Rucksack>) -> char {
        let first = input[0].clone();
        // println!("---");
        // try every char from first on ALL Rucksacks and check if each one has exactly one of these items in either compartment
        (first.left.clone() + &first.right)
            .chars()
            .find(|c| {
                // println!("{}", c);
                input.iter().all(|rucksack| {
                    let count = (rucksack.left.clone() + &rucksack.right)
                        .chars()
                        .filter(|rc| rc == c)
                        .count();
                    // println!("{}{}  '{}' => {}", rucksack.left, rucksack.right, c, count);
                    count >= 1
                })
            })
            .unwrap()
    }

    #[test]
    fn test_part2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg";
        let data = crate::parser::parse(input);

        assert_eq!('r', find_badge(&data));
    }

    #[test]
    fn test_part2_example2() {
        let input = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let data = crate::parser::parse(input);

        assert_eq!('Z', find_badge(&data));
    }
}

fn part1(input: &Vec<Rucksack>) -> u32 {
    input
        .iter()
        .map(|rucksack| {
            logic::find_duplicates(&rucksack.left, &rucksack.right)
                .iter()
                // .inspect(|c| println!("{}", c))
                .map(|c| parser::parse_priority(*c))
                .sum::<u32>()
        })
        .sum()
}

fn part2(input: &Vec<Rucksack>) -> u32 {
    input
        .chunks_exact(3)
        .map(|group| {
            let tmp = vec![group[0].clone(), group[1].clone(), group[2].clone()];
            logic::find_badge(&tmp)
        })
        .map(|c| parser::parse_priority(c))
        .sum()
}

#[test]
fn test_examples() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    let data = parser::parse(input);
    assert_eq!(157, part1(&data));
    assert_eq!(70, part2(&data));
}

fn main() {
    let input = include_str!("input.txt");
    let data = parser::parse(input);

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
