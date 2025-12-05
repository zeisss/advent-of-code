#[derive(Debug)]
enum Line {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug)]
enum ParseLineError {
    UnknownCommand,
}
use std::str::FromStr;
impl FromStr for Line {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_ascii_whitespace();
        let cmd = it.next().unwrap();
        let i = it.next().unwrap().parse::<i32>().unwrap();
        
        match cmd {
        "forward" => Ok(Line::Forward(i)),
        "up" => Ok(Line::Up(i)),
        "down" => Ok(Line::Down(i)),
        _ => Err(ParseLineError::UnknownCommand),
        }
    }
}

fn parse(s: &str) -> Vec<Line> {
    s.lines().map(|s| s.parse::<Line>().unwrap()).collect()
}

fn process1(input: &str) -> i32 {
    let (depth, forward) : (i32, i32) = parse(input).iter().fold((0,0), |(depth, forward), line| {
        match line {
        Line::Forward(f) => (depth, forward + f),
        Line::Up(d) => (depth - d, forward),
        Line::Down(d) => (depth + d, forward),
        }
    });
    depth as i32 * forward as i32
}

fn process2(input: &str) -> i32 {
    let (depth, _, forward) : (i32, i32, i32) = parse(input)
        .iter()
        .fold((0,0,0), |(depth, aim, forward), line| {
        match line {
            Line::Forward(d) => (depth + (aim * d), aim, forward + d),
            Line::Down(d) => (depth, aim + d, forward),
            Line::Up(d) => (depth, aim - d, forward),
            }
        });
    depth as i32 * forward as i32 
}

#[test]
fn test_task_example() {
    let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    assert_eq!(150, process1(input));
    assert_eq!(900, process2(input));
}

fn main() {
    let input = include_str!("input.txt");
    println!("Task 1: {:?}", process1(input));
    println!("Task 2: {:?}", process2(input));
}
