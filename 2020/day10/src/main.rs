fn parse_bag(s: &str) -> Vec<i32> {
    let mut numbers: Vec<i32> = s.split_whitespace().flat_map(|s| s.parse()).collect();
    numbers.push(0);
    numbers.push(numbers.iter().max().unwrap() + 3);
    numbers
}
fn part1(s: &str) -> i32 {
    let mut numbers = parse_bag(s);
    numbers.sort();
    println!("{:?}", numbers);

    let (diff1, diff2, diff3) =
        numbers
            .windows(2)
            .map(|a| a[1] - a[0])
            .fold((0, 0, 0), |(diff1, diff2, diff3), diff| match diff {
                1 => (diff1 + 1, diff2, diff3),
                2 => (diff1, diff2 + 1, diff3),
                3 => (diff1, diff2, diff3 + 1),
                _ => panic!("Invalid joltage difference: {}", diff),
            });
    println!("{} {} {}", diff1, diff2, diff3);
    diff1 * diff3
}

#[test]
fn test_part1() {
    let input = "16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4";
    assert_eq!(7 * 5, part1(input));
}

#[test]
fn test_part1_large() {
    let input = "28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3";
    assert_eq!(22 * 10, part1(input));
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day 10!");
    println!("Part 1: {}", part1(input));
}
