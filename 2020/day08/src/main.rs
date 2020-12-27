use std::collections::HashSet;

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

fn part1(s: &str) -> usize {
    let program : Vec<_> = s.lines() // .map(|s| s.trim())
        .map(|s| {
            let tmp = s.split_whitespace().collect::<Vec<_>>();
            println!("{:?}", tmp);
            (tmp[0], tmp[1].parse::<i32>().unwrap())
         })
        .collect();
    
    let mut acc = 0;
    let mut pc = 0;
    let mut seen = HashSet::new();

    loop {
        if seen.contains(&pc) {
            return acc
        }
        seen.insert(pc);

        let stmt = program[pc];
        match stmt.0 {
        "nop" => pc+=1,
        "acc" => {
            pc+=1;
            acc = (acc as i32 + stmt.1) as usize;
        },
        "jmp" => {
            pc = (pc as i32 + stmt.1) as usize;
        }
        _ => panic!("statement not supported: {:?}", stmt.0),
        }
    }
}

fn part2(s: &str) -> i32 {-1}

fn main() {
    println!("Day 08");
    let input = include_str!("input.txt");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}
