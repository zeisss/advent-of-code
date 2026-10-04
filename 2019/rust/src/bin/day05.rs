use aoc19::computer::*;

fn main() {
    let input = include_str!("../../inputs/day05.txt");
    let mem = parse_program(input).unwrap();

    let part1_input = vec![1];
    let part1_result = execute(mem.clone(), part1_input);
    println!("Part 1: {:?}", part1_result.output);


    let part1_input = vec![5];
    let part1_result = execute(mem, part1_input);
    println!("Part 2: {:?}", part1_result.output);

}
