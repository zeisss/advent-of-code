fn parse(s: &str) -> Vec<i32> {
    s.lines().map(|s| s.parse::<i32>().unwrap()).collect()
}

fn process1(input: &str) -> usize {
    reduce(parse(input))
}

fn process2(input: &str) -> usize {
    let n = parse(input)
        .windows(3)
        .map(|a| a.iter().sum())
        .collect::<Vec<i32>>();
    reduce(n)        
}

fn reduce(input: Vec<i32>) -> usize {
    input.windows(2)
        .filter(|n| n[0] < n[1])
        .count()
}

#[test]
fn TestTaskExample1() {
    let input = "199
200
208
210
200
207
240
269
260
263";
    assert_eq!(7, process1(input));
    assert_eq!(5, process2(input));
}

fn main() {
    let input = include_str!("input.txt");
    println!("Hello, world! {:?}", process1(input));
    println!("Task 2: {:?}", process2(input));
}
