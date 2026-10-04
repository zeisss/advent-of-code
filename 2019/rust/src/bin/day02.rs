use aoc19::computer::*;

fn run(mut mem: Memory, noun: i32, verb: i32) -> i32 {
    mem[1] = noun;
    mem[2] = verb;
    let result = execute(mem, vec![]);
    result.memory[0]
}

fn main() {
    let input = include_str!("../../inputs/day02.txt");
    let mem = parse_program(input).unwrap();

    let day01_result = run(mem.clone(), 12, 0);
    println!("Part 1: {:?}", day01_result);

    for noun in 0..=99 {
        for verb in 0..=99 {
            let result = run(mem.clone(), noun, verb);
            if result == 19690720 {
                println!(
                    "Part 2 Noun={} verb={}   => {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
                return;
            }
        }
    }
}
