
fn parse(s: &str) -> Vec<u32> {
    s.split("\n\n").map(|block| {
        block.lines().map(|line| line.parse::<u32>().unwrap()).sum()
    }).collect()
}

fn sum_topk(input: &mut Vec<u32>, n: usize) -> u32 {
    input.sort();
    input.reverse();
    input.iter().take(n).sum()
}

#[test]
fn test_foo() {
    assert_eq!(1, 1);

    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    let mut list = parse(input);
    assert_eq!(5, list.len());
    assert_eq!(6000, list[0]);
    assert_eq!(24000, *list.iter().max().unwrap());

    // part2
    assert_eq!(45000, sum_topk(&mut list, 3))
}


fn main() {
    let input = include_str!("input.txt");
    let mut data = parse(input);

    println!("Part 1: {}", *data.iter().max().unwrap());
    println!("Part 2: {}", sum_topk(&mut data, 3));
}
