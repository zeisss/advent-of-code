#[derive(Debug)]
struct Item {
    line: i32,
    size: usize,
}

use std::str::FromStr;

impl FromStr for Item {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
        Ok(i) => Ok(Item{line: i, size: s.len()}),
        Err(err) => Err(err),
        }
    }
}

impl Item {
    fn check(&self, position: usize) -> bool {
        self.line & (self.size - position) == 1
    }
}

fn parse(s: &str) -> Vec<Item> {
    s.lines().map(|s| s.parse().unwrap()).collect()
}

fn process1(input: &str) -> i64 {
    let data = parse(input);

    let columns = data.first().unwrap().size;
    let rows = data.len();
    let mut result : i64 = 0;
    
    for n in 0..columns {
        let ones = count(&data, n);
        let zeros = rows - ones;
        if ones > zeros {
            result = result * 2 + 1;
        } else {
            result = result * 2;
        }
    }
    let p = (2 as i64).pow(columns as u32);
    result * (p - 1 - result)
}

fn process2(input: &str) -> i32 {
    let data = parse(input);
    reduce(&data, BitCriteria::OxgenGeneratorRating, 0)
     * reduce(&data, BitCriteria::CO2ScrubberRating, 0)
}

enum BitCriteria {
    OxgenGeneratorRating,
    CO2ScrubberRating,
}

fn reduce(input: &Vec<Item>, bitcriteria: BitCriteria, position: usize) -> i32 {
    let ones = count(input, position);
    let zeros = input.len() - ones;

    let reduced = filter(input, position, ones >= zeros);
    if reduced.len() == 0 {
        panic!("reduced set is empty");
    }
    if reduced.len() == 1 {
    }

    
    0
}

fn filter(input: &Vec<Item>, position: usize, expected: bool) -> Vec<Item> {
    input.clone().into_iter().filter(|row| row[position] == expected).collect()
}
fn count(input: &Vec<Item>, position: usize) -> usize {
    input.iter().map(|row| row[position]).filter(|b| *b).count()
}

#[test]
fn test_task_example() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    assert_eq!(198, process1(input));

    let data = parse(input);
    assert_eq!(23, reduce(&data, BitCriteria::OxgenGeneratorRating, 0));
    assert_eq!(10, reduce(&data, BitCriteria::CO2ScrubberRating, 0));
    assert_eq!(230, process2(input));
}

fn main() {
    let input = include_str!("input.txt");
    println!("Task 1: {:?}", process1(input));
    println!("Task 2: {:?}", process2(input));
}
