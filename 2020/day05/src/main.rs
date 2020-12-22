



fn seat_id(row: i32, col: i32) -> i32 {
    row * 8 + col
}

#[test]
fn test_seat_id() {
    assert_eq!(567, seat_id(70, 7));
}

fn to_row(s: &str) -> i32 {
    // row: 0-127
    // F = lower half
    // B = upper half
    let row_range = s.chars().take(7)
    .fold((0, 127), |(min, max), c| {
        println!("({}, {}) + {}", min, max, c);
        match c {
            'F' => (min, (max as f64 - ((max-min) as f64)/2.0).floor() as i32),
            'B' => ((min as f64 + (max-min) as f64 / 2.0).ceil() as i32, max),
            _ => panic!("unknown: {}", c),
        }
    });
    assert_eq!(row_range.0, row_range.1);
    row_range.0
}

#[test]
fn test_to_row() {
    assert_eq!(44, to_row("FBFBBFFRLR"));
    assert_eq!(0, to_row("FFFFFFF___"));
    assert_eq!(127, to_row("BBBBBBB___"));
}

fn to_col(s: &str) -> i32 {
    // seat: 0-7
    // R = upper half
    // L = lower half
    let row_range = s.chars().skip(7).take(3)
    .fold((0, 7), |(min, max), c| {
        println!("({}, {}) + {}", min, max, c);
        match c {
            'L' => (min, (max as f64 - ((max-min) as f64)/2.0).floor() as i32),
            'R' => ((min as f64 + (max-min) as f64 / 2.0).ceil() as i32, max),
            _ => panic!("unknown: {}", c),
        }
    });
    assert_eq!(row_range.0, row_range.1);
    row_range.0
}


#[test]
fn test_to_col() {
    assert_eq!(0, to_col("_______LLL"));
    assert_eq!(7, to_col("_______RRR"));
    assert_eq!(5, to_col("FBFBBFFRLR"));
}

fn parse_boardingpass(s: &str) -> i32 {
    let row = to_row(s);
    let col = to_col(s);
    seat_id(row, col)
}

#[test]
fn test_parse_boardingpass() {
    assert_eq!(567, parse_boardingpass("BFFFBBFRRR"));
    assert_eq!(119, parse_boardingpass("FFFBBBFRRR"));
    assert_eq!(820, parse_boardingpass("BBFFBBFRLL"));
}

fn part1(s: &str) -> i32 {
    s.lines().map(|s| parse_boardingpass(s)).max().unwrap_or(0)
}


fn main() {
    let input = include_str!("input.txt");
    println!("Day 01!");
    println!("Part 1: {}", part1(input)); // 292 is too low
    println!("Part 2: {}", part2(input));
}

fn part2(_s: &str) -> i32 { 0 }
