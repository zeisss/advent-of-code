use std::collections::HashSet;

enum ExecutionResult {
    Exit(i32),
    InfiniteLoop(i32),
}

fn parse(s: &str) -> Vec<(&str, i32)> {
    s.trim().lines()
        .map(|s| {
            let tmp = s.split_whitespace().collect::<Vec<_>>();
            (tmp[0], tmp[1].parse::<i32>().unwrap())
        })
        .collect()
}

fn execute(program: Vec<(&str, i32)>) -> ExecutionResult {
    use ExecutionResult::*;

    let mut acc = 0;
    let mut pc = 0;
    let mut seen = HashSet::new();

    loop {
        if seen.contains(&pc) {
            return InfiniteLoop(acc);
        }
        seen.insert(pc);

        if let Some(stmt) = program.get(pc) {
            println!("Executing {:?}", stmt);
            match stmt.0 {
                "nop" => pc += 1,
                "acc" => {
                    pc += 1;
                    acc = acc + stmt.1;
                }
                "jmp" => {
                    pc = (pc as i32 + stmt.1) as usize;
                }
                _ => panic!("statement not supported: {:?}", stmt.0),
            }
        } else {
            return Exit(acc);
        }
    }
}

#[test]
fn test_part1() {
    let input = "nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6
    ";

    let acc = part1(input);
    assert_eq!(5, acc);
}

fn part1(s: &str) -> i32 {
    let program: Vec<_> = parse(s);
    match execute(program) {
    ExecutionResult::InfiniteLoop(r) => r,
    ExecutionResult::Exit(r) => r,
    }
}

#[test]
fn test_part2() {
    let input = "nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6
    ";

    let acc = part2(input);
    assert_eq!(8, acc);
}

fn part2(s: &str) -> i32 {
    let program = parse(s);

    let result = (0..program.len())
        .map(|i| {
            let mut fixed = program.clone();
            let f = fixed.get_mut(i).unwrap();
            match f.0 {
                "nop" => f.0 = "jmp",
                "jmp" => f.0 = "nop",
                _ => {}
            };
            fixed
        })
        .map(|p| execute(p))
        .find(|result| {
            if let ExecutionResult::Exit(_) = result {
                true
            } else {
                false
            }
        })
        .unwrap();
    
    match result {
    ExecutionResult::InfiniteLoop(r) => panic!("infinite loop: {}", r),
    ExecutionResult::Exit(r) => r,
    }
}

fn main() {
    println!("Day 08");
    let input = include_str!("input.txt");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}
